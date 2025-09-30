use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::process;
use std::sync::Arc;

use rustls::{ClientConfig, ClientConnection, StreamOwned};
use webpki_roots::TLS_SERVER_ROOTS;

use pushover::{load_config, parse_url, url_encode, Config};

const PUSHOVER_API_URL: &str = "https://api.pushover.net/1/messages.json";

fn usage() {
    let program_name = env::args().next().unwrap_or_else(|| "pushover".to_string());
    eprintln!("Usage: {} -t <title> -m <message> [OPTIONS]", program_name);
    eprintln!("  -t <title>      Title of the notification");
    eprintln!("  -m <message>    Message of the notification");
    eprintln!("  -p <priority>   Priority (-2 to 2, default: 0)");
    eprintln!("  --app-token <token>  Override app token from config");
    eprintln!("  -h, --help      Show this help message");
    eprintln!();
    eprintln!("Configuration:");
    eprintln!("  Reads configuration from /etc/pushover/config.toml");
    eprintln!("  Falls back to etc/pushover/config.toml for development");
    process::exit(1);
}

fn send_notification_rustls(
    config: &Config,
    title: &str,
    message: &str,
    priority: i8,
    app_token_override: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (host, port, path) = parse_url(PUSHOVER_API_URL)?;

    // Build form data
    let token = app_token_override.unwrap_or(&config.pushover.token);
    let mut form_parts = vec![
        format!("token={}", url_encode(token)),
        format!("user={}", url_encode(&config.pushover.user)),
        format!("title={}", url_encode(title)),
        format!("message={}", url_encode(message)),
    ];

    // Add priority if not default
    if priority != 0 {
        form_parts.push(format!("priority={}", priority));
    }

    // Add optional notification settings
    if let Some(notification) = &config.notification {
        if let Some(sound) = &notification.sound {
            form_parts.push(format!("sound={}", url_encode(sound)));
        }
        if let Some(device) = &notification.device {
            form_parts.push(format!("device={}", url_encode(device)));
        }
    }

    let form_data = form_parts.join("&");

    // Create TLS config
    let mut root_store = rustls::RootCertStore::empty();
    root_store.add_trust_anchors(TLS_SERVER_ROOTS.iter().map(|ta| {
        rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject,
            ta.spki,
            ta.name_constraints,
        )
    }));

    let config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    // Connect to server
    let server_name = rustls::ServerName::try_from(host.as_str())?;
    let conn = ClientConnection::new(Arc::new(config), server_name)?;
    let sock = TcpStream::connect(format!("{}:{}", host, port))?;
    let mut tls = StreamOwned::new(conn, sock);

    // Build HTTP request
    let request = format!(
        "POST {} HTTP/1.1\r\n\
         Host: {}\r\n\
         Content-Type: application/x-www-form-urlencoded\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         User-Agent: pushover-rust/1.0\r\n\
         \r\n\
         {}",
        path,
        host,
        form_data.len(),
        form_data
    );

    // Send request
    tls.write_all(request.as_bytes())?;

    // Read response
    let mut response = Vec::new();
    tls.read_to_end(&mut response)?;

    // Parse response to check for errors
    let response_str = String::from_utf8_lossy(&response);
    if let Some(status_line) = response_str.lines().next() {
        if !status_line.contains("200") {
            return Err(format!("HTTP request failed: {}", status_line).into());
        }
    }

    Ok(())
}

fn main() {
    // Load configuration
    let config = match load_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading configuration: {}", e);
            eprintln!("Please ensure /etc/pushover/config.toml exists and is properly configured.");
            process::exit(1);
        }
    };

    // Determine default title
    let default_title = config.pushover.default_title.clone().unwrap_or_else(|| {
        format!(
            "{} @",
            env::var("HOSTNAME").unwrap_or_else(|_| "localhost".to_string())
        )
    });

    let mut title = default_title;
    let mut message = String::new();
    let mut priority: i8 = 0;
    let mut app_token_override: Option<String> = None;

    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "-t" => {
                if i + 1 >= args.len() {
                    eprintln!("Option -t requires an argument.");
                    usage();
                }
                title = args[i + 1].clone();
                i += 2;
            }
            "-m" => {
                if i + 1 >= args.len() {
                    eprintln!("Option -m requires an argument.");
                    usage();
                }
                message = args[i + 1].clone();
                i += 2;
            }
            "-p" => {
                if i + 1 >= args.len() {
                    eprintln!("Option -p requires an argument.");
                    usage();
                }
                match args[i + 1].parse::<i8>() {
                    Ok(p) if (-2..=2).contains(&p) => priority = p,
                    Ok(_) => {
                        eprintln!("Priority must be between -2 and 2.");
                        usage();
                    }
                    Err(_) => {
                        eprintln!("Priority must be a valid integer.");
                        usage();
                    }
                };
                i += 2;
            }
            "--app-token" => {
                if i + 1 >= args.len() {
                    eprintln!("Option --app-token requires an argument.");
                    usage();
                }
                app_token_override = Some(args[i + 1].clone());
                i += 2;
            }
            "-h" | "--help" => {
                usage();
            }
            arg if arg.starts_with('-') => {
                eprintln!("Invalid option {}", arg);
                usage();
            }
            _ => {
                eprintln!("Unexpected argument: {}", args[i]);
                usage();
            }
        }
    }

    // Check if message is provided
    if message.is_empty() {
        eprintln!("Message is required.");
        usage();
    }

    // Send the notification
    match send_notification_rustls(
        &config,
        &title,
        &message,
        priority,
        app_token_override.as_deref(),
    ) {
        Ok(()) => {
            // Success - silent like the original script
        }
        Err(e) => {
            eprintln!("Error sending notification: {}", e);
            process::exit(1);
        }
    }
}
