use std::env;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::process;
use std::sync::Arc;

use rustls::{ClientConfig, ClientConnection, StreamOwned};
use serde::{Deserialize, Serialize};
use webpki_roots::TLS_SERVER_ROOTS;

#[derive(Debug, Deserialize, Serialize)]
struct PushoverConfig {
    user: String,
    token: String,
    #[serde(default)]
    default_title: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct NotificationConfig {
    #[serde(default)]
    sound: Option<String>,
    #[serde(default)]
    device: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    pushover: PushoverConfig,
    #[serde(default)]
    notification: Option<NotificationConfig>,
}

const PUSHOVER_API_URL: &str = "https://api.pushover.net/1/messages.json";

fn usage() {
    let program_name = env::args().next().unwrap_or_else(|| "pushover".to_string());
    eprintln!("Usage: {} -t <title> -m <message> [OPTIONS]", program_name);
    eprintln!("  -t <title>      Title of the notification");
    eprintln!("  -m <message>    Message of the notification");
    eprintln!("  -p <priority>   Priority (-2 to 2, default: 0)");
    eprintln!("  -h, --help      Show this help message");
    eprintln!();
    eprintln!("Configuration:");
    eprintln!("  Reads configuration from /etc/pushover/config.toml");
    eprintln!("  Falls back to etc/pushover/config.toml for development");
    process::exit(1);
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    // Try system config first, then fallback to local config for development
    let system_config = "/etc/pushover/config.toml";
    let local_config = "etc/pushover/config.toml";

    let (config_path, config_content) = if let Ok(content) = fs::read_to_string(system_config) {
        (system_config, content)
    } else if let Ok(content) = fs::read_to_string(local_config) {
        (local_config, content)
    } else {
        return Err(format!(
            "Config file not found. Tried {} and {}",
            system_config, local_config
        )
        .into());
    };

    let config: Config = toml::from_str(&config_content)
        .map_err(|e| format!("Invalid TOML in config file {}: {}", config_path, e))?;

    Ok(config)
}

fn url_encode(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            ' ' => "+".to_string(),
            _ => {
                let bytes = c.to_string().into_bytes();
                bytes.iter().map(|b| format!("%{:02X}", b)).collect()
            }
        })
        .collect()
}

fn parse_url(url: &str) -> Result<(String, u16, String), Box<dyn std::error::Error>> {
    if !url.starts_with("https://") {
        return Err("Only HTTPS URLs are supported".into());
    }

    let url_without_scheme = &url[8..]; // Remove "https://"
    let parts: Vec<&str> = url_without_scheme.splitn(2, '/').collect();

    let host_port = parts[0];
    let path = if parts.len() > 1 {
        format!("/{}", parts[1])
    } else {
        "/".to_string()
    };

    let (host, port) = if host_port.contains(':') {
        let host_port_parts: Vec<&str> = host_port.splitn(2, ':').collect();
        (host_port_parts[0].to_string(), host_port_parts[1].parse()?)
    } else {
        (host_port.to_string(), 443)
    };

    Ok((host, port, path))
}

fn send_notification_rustls(
    config: &Config,
    title: &str,
    message: &str,
    priority: i8,
) -> Result<(), Box<dyn std::error::Error>> {
    let (host, port, path) = parse_url(PUSHOVER_API_URL)?;

    // Build form data
    let mut form_parts = vec![
        format!("token={}", url_encode(&config.pushover.token)),
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
    match send_notification_rustls(&config, &title, &message, priority) {
        Ok(()) => {
            // Success - silent like the original script
        }
        Err(e) => {
            eprintln!("Error sending notification: {}", e);
            process::exit(1);
        }
    }
}
