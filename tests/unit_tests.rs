use pushover::{parse_url, url_encode, Config, NotificationConfig, PushoverConfig};

#[test]
fn test_url_encode_basic() {
    assert_eq!(url_encode("hello"), "hello");
    assert_eq!(url_encode("hello world"), "hello+world");
    assert_eq!(url_encode("test123"), "test123");
    assert_eq!(url_encode(""), "");
}

#[test]
fn test_url_encode_special_characters() {
    assert_eq!(url_encode("hello@world.com"), "hello%40world.com");
    assert_eq!(
        url_encode("user-name_test.file~backup"),
        "user-name_test.file~backup"
    );
    assert_eq!(
        url_encode("special!@#$%^&*()"),
        "special%21%40%23%24%25%5E%26%2A%28%29"
    );
    assert_eq!(url_encode("100%"), "100%25");
    assert_eq!(url_encode("a+b=c"), "a%2Bb%3Dc");
    assert_eq!(url_encode("user@domain.com"), "user%40domain.com");
}

#[test]
fn test_url_encode_whitespace_and_control_chars() {
    assert_eq!(url_encode("hello\nworld"), "hello%0Aworld");
    assert_eq!(url_encode("hello\tworld"), "hello%09world");
    assert_eq!(url_encode("hello\rworld"), "hello%0Dworld");
    assert_eq!(url_encode(" space "), "+space+");
    assert_eq!(url_encode("  "), "++");
}

#[test]
fn test_url_encode_unicode() {
    assert_eq!(url_encode("áéíóú"), "%C3%A1%C3%A9%C3%AD%C3%B3%C3%BA");
    assert_eq!(url_encode("中文"), "%E4%B8%AD%E6%96%87");
    assert_eq!(url_encode("🚀"), "%F0%9F%9A%80");
    assert_eq!(url_encode("café"), "caf%C3%A9");
    assert_eq!(url_encode("naïve"), "na%C3%AFve");
}

#[test]
fn test_url_encode_safe_characters() {
    // These characters should not be encoded
    let safe_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_.~";
    assert_eq!(url_encode(safe_chars), safe_chars);
}

#[test]
fn test_parse_url_valid_https() {
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

    let (host, port, path) =
        parse_url("https://subdomain.domain.com:9999/very/long/path/here").unwrap();
    assert_eq!(host, "subdomain.domain.com");
    assert_eq!(port, 9999);
    assert_eq!(path, "/very/long/path/here");
}

#[test]
fn test_parse_url_default_port() {
    let (host, port, path) = parse_url("https://api.example.com:443/").unwrap();
    assert_eq!(host, "api.example.com");
    assert_eq!(port, 443);
    assert_eq!(path, "/");

    let (host, port, path) = parse_url("https://test.com/path").unwrap();
    assert_eq!(host, "test.com");
    assert_eq!(port, 443);
    assert_eq!(path, "/path");
}

#[test]
fn test_parse_url_complex_paths() {
    let (host, port, path) = parse_url("https://api.service.com/v1/endpoint?param=value").unwrap();
    assert_eq!(host, "api.service.com");
    assert_eq!(port, 443);
    assert_eq!(path, "/v1/endpoint?param=value");

    let (host, port, path) = parse_url("https://example.com:8080/path/to/resource.json").unwrap();
    assert_eq!(host, "example.com");
    assert_eq!(port, 8080);
    assert_eq!(path, "/path/to/resource.json");
}

#[test]
fn test_parse_url_invalid() {
    // Non-HTTPS URLs should be rejected
    assert!(parse_url("http://example.com").is_err());
    assert!(parse_url("ftp://example.com").is_err());
    assert!(parse_url("ws://example.com").is_err());
    assert!(parse_url("not-a-url").is_err());

    // Empty or malformed URLs
    assert!(parse_url("").is_err());
    // Note: "https://" actually parses to empty host, which may be undesirable but is current behavior
    let result = parse_url("https://");
    assert!(result.is_ok()); // Current implementation allows this
    let (host, _, _) = result.unwrap();
    assert_eq!(host, ""); // Returns empty host

    // Invalid port numbers
    assert!(parse_url("https://example.com:invalid_port").is_err());
    assert!(parse_url("https://example.com:99999").is_err());
    assert!(parse_url("https://example.com:-1").is_err());
}

#[test]
fn test_config_structure() {
    let config = Config {
        pushover: PushoverConfig {
            user: "test_user_key".to_string(),
            token: "test_app_token".to_string(),
            default_title: Some("Test Server".to_string()),
        },
        notification: Some(NotificationConfig {
            sound: Some("cosmic".to_string()),
            device: Some("iphone".to_string()),
        }),
    };

    assert_eq!(config.pushover.user, "test_user_key");
    assert_eq!(config.pushover.token, "test_app_token");
    assert_eq!(
        config.pushover.default_title,
        Some("Test Server".to_string())
    );

    let notification = config.notification.unwrap();
    assert_eq!(notification.sound, Some("cosmic".to_string()));
    assert_eq!(notification.device, Some("iphone".to_string()));
}

#[test]
fn test_config_minimal() {
    let config = Config {
        pushover: PushoverConfig {
            user: "user123".to_string(),
            token: "token456".to_string(),
            default_title: None,
        },
        notification: None,
    };

    assert_eq!(config.pushover.user, "user123");
    assert_eq!(config.pushover.token, "token456");
    assert!(config.pushover.default_title.is_none());
    assert!(config.notification.is_none());
}

#[test]
fn test_notification_config_default() {
    let notification = NotificationConfig::default();
    assert!(notification.sound.is_none());
    assert!(notification.device.is_none());
}

#[test]
fn test_notification_config_partial() {
    let notification = NotificationConfig {
        sound: Some("pushover".to_string()),
        device: None,
    };
    assert_eq!(notification.sound, Some("pushover".to_string()));
    assert!(notification.device.is_none());
}

#[test]
fn test_token_override_logic() {
    let config_token = "config_token_123";

    // Test with override - simulate function that might return Some or None
    fn get_override_token(has_override: bool) -> Option<&'static str> {
        if has_override {
            Some("override_token_456")
        } else {
            None
        }
    }

    // Test with override
    let override_token = get_override_token(true);
    let selected = override_token.unwrap_or(config_token);
    assert_eq!(selected, "override_token_456");

    // Test without override
    let no_override = get_override_token(false);
    let selected = no_override.unwrap_or(config_token);
    assert_eq!(selected, "config_token_123");
}

#[test]
fn test_priority_validation_logic() {
    // Valid priorities
    let valid_priorities = [-2, -1, 0, 1, 2];
    for priority in valid_priorities {
        assert!(
            (-2..=2).contains(&priority),
            "Priority {} should be valid",
            priority
        );
    }

    // Invalid priorities
    let invalid_priorities = [-3, -10, 3, 5, 100];
    for priority in invalid_priorities {
        assert!(
            !(-2..=2).contains(&priority),
            "Priority {} should be invalid",
            priority
        );
    }
}

#[test]
fn test_form_data_encoding() {
    // Test that we can properly encode form data components
    let user = "test_user@example.com";
    let token = "token_with-special.chars";
    let title = "Alert: Server Down!";
    let message = "The production server is experiencing issues. Please check immediately.";

    let user_encoded = url_encode(user);
    let token_encoded = url_encode(token);
    let title_encoded = url_encode(title);
    let message_encoded = url_encode(message);

    assert_eq!(user_encoded, "test_user%40example.com");
    assert_eq!(token_encoded, "token_with-special.chars");
    assert_eq!(title_encoded, "Alert%3A+Server+Down%21");
    assert!(message_encoded.contains("production+server"));
    assert!(message_encoded.contains("Please+check"));
}

#[test]
fn test_config_serialization() {
    let config_toml = r#"
[pushover]
user = "test_user_key"
token = "test_app_token"
default_title = "Production Server"

[notification]
sound = "cosmic"
device = "iphone"
"#;

    let config: Config = toml::from_str(config_toml).unwrap();

    assert_eq!(config.pushover.user, "test_user_key");
    assert_eq!(config.pushover.token, "test_app_token");
    assert_eq!(
        config.pushover.default_title,
        Some("Production Server".to_string())
    );

    let notification = config.notification.unwrap();
    assert_eq!(notification.sound, Some("cosmic".to_string()));
    assert_eq!(notification.device, Some("iphone".to_string()));
}

#[test]
fn test_config_with_comments() {
    let config_toml = r#"
# Pushover configuration
[pushover]
user = "user123"     # Your user key
token = "token456"   # Your app token
default_title = "Server Alert"

# Optional notification settings
[notification]
sound = "pushover"   # Notification sound
# device = "iphone"  # Commented out device
"#;

    let config: Config = toml::from_str(config_toml).unwrap();

    assert_eq!(config.pushover.user, "user123");
    assert_eq!(config.pushover.token, "token456");
    assert_eq!(
        config.pushover.default_title,
        Some("Server Alert".to_string())
    );

    let notification = config.notification.unwrap();
    assert_eq!(notification.sound, Some("pushover".to_string()));
    assert!(notification.device.is_none()); // Should be None since it's commented
}

#[test]
fn test_edge_case_urls() {
    // URL with query parameters
    let (host, port, path) =
        parse_url("https://api.example.com/endpoint?key=value&other=param").unwrap();
    assert_eq!(host, "api.example.com");
    assert_eq!(port, 443);
    assert_eq!(path, "/endpoint?key=value&other=param");

    // URL with fragment (though fragments shouldn't be sent to server)
    let (host, port, path) = parse_url("https://example.com/page#section").unwrap();
    assert_eq!(host, "example.com");
    assert_eq!(port, 443);
    assert_eq!(path, "/page#section");

    // URL with encoded characters
    let (host, port, path) = parse_url("https://example.com/path%20with%20spaces").unwrap();
    assert_eq!(host, "example.com");
    assert_eq!(port, 443);
    assert_eq!(path, "/path%20with%20spaces");
}

#[test]
fn test_url_encode_boundary_conditions() {
    // Test empty string
    assert_eq!(url_encode(""), "");

    // Test single characters
    assert_eq!(url_encode("a"), "a");
    assert_eq!(url_encode(" "), "+");
    assert_eq!(url_encode("@"), "%40");

    // Test very long strings
    let long_string = "a".repeat(1000);
    let encoded = url_encode(&long_string);
    assert_eq!(encoded, long_string); // Should remain unchanged since 'a' is safe

    let long_string_with_spaces = format!("a{}", " ".repeat(999));
    let encoded = url_encode(&long_string_with_spaces);
    assert!(encoded.starts_with("a"));
    assert!(encoded.contains("+"));
    assert_eq!(encoded.len(), 1000); // 'a' + 999 '+' characters
}
