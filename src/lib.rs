use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PushoverConfig {
    pub user: String,
    pub token: String,
    #[serde(default)]
    pub default_title: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct NotificationConfig {
    #[serde(default)]
    pub sound: Option<String>,
    #[serde(default)]
    pub device: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub pushover: PushoverConfig,
    #[serde(default)]
    pub notification: Option<NotificationConfig>,
}

pub fn url_encode(s: &str) -> String {
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

pub fn parse_url(url: &str) -> Result<(String, u16, String), Box<dyn std::error::Error>> {
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

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    use std::fs;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_encode() {
        assert_eq!(url_encode("hello"), "hello");
        assert_eq!(url_encode("hello world"), "hello+world");
        assert_eq!(url_encode("hello@world.com"), "hello%40world.com");
        assert_eq!(url_encode("test123"), "test123");
        assert_eq!(
            url_encode("user-name_test.file~backup"),
            "user-name_test.file~backup"
        );
        assert_eq!(
            url_encode("special!@#$%^&*()"),
            "special%21%40%23%24%25%5E%26%2A%28%29"
        );
        assert_eq!(url_encode(""), "");
        assert_eq!(url_encode("áéíóú"), "%C3%A1%C3%A9%C3%AD%C3%B3%C3%BA");
    }

    #[test]
    fn test_parse_url() {
        // Valid HTTPS URLs
        let (host, port, path) = parse_url("https://api.pushover.net/1/messages.json").unwrap();
        assert_eq!(host, "api.pushover.net");
        assert_eq!(port, 443);
        assert_eq!(path, "/1/messages.json");

        let (host, port, path) = parse_url("https://example.com:8443/api/test").unwrap();
        assert_eq!(host, "example.com");
        assert_eq!(port, 8443);
        assert_eq!(path, "/api/test");

        let (host, port, path) = parse_url("https://example.com").unwrap();
        assert_eq!(host, "example.com");
        assert_eq!(port, 443);
        assert_eq!(path, "/");

        // Invalid URLs
        assert!(parse_url("http://example.com").is_err());
        assert!(parse_url("ftp://example.com").is_err());
        assert!(parse_url("not-a-url").is_err());
    }

    #[test]
    fn test_config_parsing() {
        let config_content = r#"
[pushover]
user = "test_user_key"
token = "test_app_token"
default_title = "Test Title"

[notification]
sound = "pushover"
device = "iphone"
"#;

        let config: Config = toml::from_str(config_content).unwrap();
        assert_eq!(config.pushover.user, "test_user_key");
        assert_eq!(config.pushover.token, "test_app_token");
        assert_eq!(
            config.pushover.default_title,
            Some("Test Title".to_string())
        );
        assert!(config.notification.is_some());
        let notification = config.notification.unwrap();
        assert_eq!(notification.sound, Some("pushover".to_string()));
        assert_eq!(notification.device, Some("iphone".to_string()));
    }

    #[test]
    fn test_notification_config_defaults() {
        let config = NotificationConfig::default();
        assert!(config.sound.is_none());
        assert!(config.device.is_none());
    }
}
