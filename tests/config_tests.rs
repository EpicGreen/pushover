use pushover::{Config, NotificationConfig};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_valid_minimal_config() {
    let config_content = r#"
[pushover]
user = "uQiRzpo4DXghDmr9QzzfQu27cmVRsG"
token = "azGDORePK8gMaC0QOYAMyEEuzJnyUi"
"#;

    let config: Config = toml::from_str(config_content).unwrap();

    assert_eq!(config.pushover.user, "uQiRzpo4DXghDmr9QzzfQu27cmVRsG");
    assert_eq!(config.pushover.token, "azGDORePK8gMaC0QOYAMyEEuzJnyUi");
    assert!(config.pushover.default_title.is_none());
    assert!(config.notification.is_none());
}

#[test]
fn test_valid_complete_config() {
    let config_content = r#"
[pushover]
user = "uQiRzpo4DXghDmr9QzzfQu27cmVRsG"
token = "azGDORePK8gMaC0QOYAMyEEuzJnyUi"
default_title = "Production Server"

[notification]
sound = "cosmic"
device = "iphone"
"#;

    let config: Config = toml::from_str(config_content).unwrap();

    assert_eq!(config.pushover.user, "uQiRzpo4DXghDmr9QzzfQu27cmVRsG");
    assert_eq!(config.pushover.token, "azGDORePK8gMaC0QOYAMyEEuzJnyUi");
    assert_eq!(
        config.pushover.default_title,
        Some("Production Server".to_string())
    );

    let notification = config.notification.unwrap();
    assert_eq!(notification.sound, Some("cosmic".to_string()));
    assert_eq!(notification.device, Some("iphone".to_string()));
}

#[test]
fn test_config_with_partial_notification() {
    let config_content = r#"
[pushover]
user = "test_user"
token = "test_token"

[notification]
sound = "pushover"
"#;

    let config: Config = toml::from_str(config_content).unwrap();

    let notification = config.notification.unwrap();
    assert_eq!(notification.sound, Some("pushover".to_string()));
    assert!(notification.device.is_none());
}

#[test]
fn test_config_missing_user() {
    let config_content = r#"
[pushover]
token = "azGDORePK8gMaC0QOYAMyEEuzJnyUi"
"#;

    let result = toml::from_str::<Config>(config_content);
    assert!(result.is_err());
}

#[test]
fn test_config_missing_token() {
    let config_content = r#"
[pushover]
user = "uQiRzpo4DXghDmr9QzzfQu27cmVRsG"
"#;

    let result = toml::from_str::<Config>(config_content);
    assert!(result.is_err());
}

#[test]
fn test_config_missing_pushover_section() {
    let config_content = r#"
[notification]
sound = "pushover"
"#;

    let result = toml::from_str::<Config>(config_content);
    assert!(result.is_err());
}

#[test]
fn test_config_empty_file() {
    let config_content = "";

    let result = toml::from_str::<Config>(config_content);
    assert!(result.is_err());
}

#[test]
fn test_config_invalid_toml_syntax() {
    let invalid_configs = vec![
        r#"
[pushover
user = "test_user"
token = "test_token"
"#, // Missing closing bracket
        r#"
[pushover]
user = test_user
token = "test_token"
"#, // Missing quotes
        r#"
[pushover]
user = "test_user"
token = "test_token
"#, // Missing closing quote
        r#"
pushover.user = "test_user"
pushover.token = "test_token"
invalid_line_without_value
"#, // Invalid syntax
    ];

    for invalid_config in invalid_configs {
        let result = toml::from_str::<Config>(invalid_config);
        assert!(
            result.is_err(),
            "Should fail to parse invalid TOML: {}",
            invalid_config
        );
    }
}

#[test]
fn test_config_empty_strings() {
    let config_content = r#"
[pushover]
user = ""
token = ""
"#;

    // Empty strings should be valid TOML but may cause issues at runtime
    let config: Config = toml::from_str(config_content).unwrap();
    assert_eq!(config.pushover.user, "");
    assert_eq!(config.pushover.token, "");
}

#[test]
fn test_config_with_comments() {
    let config_content = r#"
# This is a comment
[pushover]
user = "uQiRzpo4DXghDmr9QzzfQu27cmVRsG"  # User key comment
token = "azGDORePK8gMaC0QOYAMyEEuzJnyUi"
default_title = "Server Alert"

# Notification settings
[notification]
sound = "pushover"  # Default sound
device = "iphone"   # Target device
"#;

    let config: Config = toml::from_str(config_content).unwrap();

    assert_eq!(config.pushover.user, "uQiRzpo4DXghDmr9QzzfQu27cmVRsG");
    assert_eq!(config.pushover.token, "azGDORePK8gMaC0QOYAMyEEuzJnyUi");
    assert_eq!(
        config.pushover.default_title,
        Some("Server Alert".to_string())
    );

    let notification = config.notification.unwrap();
    assert_eq!(notification.sound, Some("pushover".to_string()));
    assert_eq!(notification.device, Some("iphone".to_string()));
}

#[test]
fn test_config_special_characters() {
    let config_content = r#"
[pushover]
user = "user_with.dots-and_underscores123"
token = "token-with-dashes_and_numbers456"
default_title = "Server @ Production Environment"

[notification]
sound = "bike"
device = "user's iPhone"
"#;

    let config: Config = toml::from_str(config_content).unwrap();

    assert_eq!(config.pushover.user, "user_with.dots-and_underscores123");
    assert_eq!(config.pushover.token, "token-with-dashes_and_numbers456");
    assert_eq!(
        config.pushover.default_title,
        Some("Server @ Production Environment".to_string())
    );

    let notification = config.notification.unwrap();
    assert_eq!(notification.sound, Some("bike".to_string()));
    assert_eq!(notification.device, Some("user's iPhone".to_string()));
}

#[test]
fn test_notification_config_default() {
    let notification = NotificationConfig::default();
    assert!(notification.sound.is_none());
    assert!(notification.device.is_none());
}

#[test]
fn test_config_file_loading_integration() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.toml");

    let config_content = r#"
[pushover]
user = "integration_test_user"
token = "integration_test_token"
default_title = "Integration Test"

[notification]
sound = "cosmic"
device = "test_device"
"#;

    fs::write(&config_path, config_content).unwrap();

    // Read and parse the file
    let file_content = fs::read_to_string(&config_path).unwrap();
    let config: Config = toml::from_str(&file_content).unwrap();

    assert_eq!(config.pushover.user, "integration_test_user");
    assert_eq!(config.pushover.token, "integration_test_token");
    assert_eq!(
        config.pushover.default_title,
        Some("Integration Test".to_string())
    );

    let notification = config.notification.unwrap();
    assert_eq!(notification.sound, Some("cosmic".to_string()));
    assert_eq!(notification.device, Some("test_device".to_string()));
}

#[test]
fn test_config_unicode_support() {
    let config_content = r#"
[pushover]
user = "test_user_ñáéíóú"
token = "test_token_中文"
default_title = "🚨 Alert émergence"

[notification]
sound = "pushover"
device = "José's iPhone"
"#;

    let config: Config = toml::from_str(config_content).unwrap();

    assert_eq!(config.pushover.user, "test_user_ñáéíóú");
    assert_eq!(config.pushover.token, "test_token_中文");
    assert_eq!(
        config.pushover.default_title,
        Some("🚨 Alert émergence".to_string())
    );

    let notification = config.notification.unwrap();
    assert_eq!(notification.sound, Some("pushover".to_string()));
    assert_eq!(notification.device, Some("José's iPhone".to_string()));
}
