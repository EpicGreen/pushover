use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

// Helper function to get the path to our binary
fn get_binary_path() -> PathBuf {
    let mut path = env::current_exe().unwrap();
    path.pop(); // Remove the test executable name
    if path.ends_with("deps") {
        path.pop(); // Remove "deps" directory
    }
    path.push("pushover");
    path
}

// Helper function to create a temporary config file
fn create_test_config(temp_dir: &TempDir) -> PathBuf {
    let config_path = temp_dir.path().join("config.toml");
    let config_content = r#"
[pushover]
user = "test_user_key_12345"
token = "test_app_token_67890"
default_title = "Test Server"

[notification]
sound = "pushover"
device = "test_device"
"#;
    fs::write(&config_path, config_content).unwrap();
    config_path
}

#[test]
fn test_help_message() {
    let output = Command::new(get_binary_path())
        .arg("--help")
        .output()
        .expect("Failed to execute binary");

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Check that help message contains expected elements
    assert!(stderr.contains("Usage:"));
    assert!(stderr.contains("-t <title>"));
    assert!(stderr.contains("-m <message>"));
    assert!(stderr.contains("-p <priority>"));
    assert!(stderr.contains("--app-token"));
    assert!(stderr.contains("-h, --help"));
}

#[test]
fn test_missing_message_error() {
    let temp_dir = TempDir::new().unwrap();
    let _config_path = create_test_config(&temp_dir);

    // Set environment variables to point to our test config
    let output = Command::new(get_binary_path())
        .env("HOME", temp_dir.path())
        .arg("-t")
        .arg("Test Title")
        // Missing -m argument
        .output()
        .expect("Failed to execute binary");

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should fail due to missing message
    assert!(!output.status.success());
    assert!(stderr.contains("Message is required") || stderr.contains("Usage:"));
}

#[test]
fn test_invalid_priority() {
    let temp_dir = TempDir::new().unwrap();
    let _config_path = create_test_config(&temp_dir);

    let output = Command::new(get_binary_path())
        .env("HOME", temp_dir.path())
        .arg("-t")
        .arg("Test Title")
        .arg("-m")
        .arg("Test Message")
        .arg("-p")
        .arg("5") // Invalid priority (must be -2 to 2)
        .output()
        .expect("Failed to execute binary");

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(!output.status.success());
    assert!(stderr.contains("Priority must be between -2 and 2") || stderr.contains("Usage:"));
}

#[test]
fn test_invalid_priority_non_numeric() {
    let temp_dir = TempDir::new().unwrap();
    let _config_path = create_test_config(&temp_dir);

    let output = Command::new(get_binary_path())
        .env("HOME", temp_dir.path())
        .arg("-t")
        .arg("Test Title")
        .arg("-m")
        .arg("Test Message")
        .arg("-p")
        .arg("abc") // Non-numeric priority
        .output()
        .expect("Failed to execute binary");

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(!output.status.success());
    assert!(stderr.contains("Priority must be a valid integer") || stderr.contains("Usage:"));
}

#[test]
fn test_missing_argument_for_flag() {
    let temp_dir = TempDir::new().unwrap();
    let _config_path = create_test_config(&temp_dir);

    // Test missing argument for -t
    let output = Command::new(get_binary_path())
        .env("HOME", temp_dir.path())
        .arg("-t")
        // Missing title argument
        .output()
        .expect("Failed to execute binary");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success());
    assert!(stderr.contains("Option -t requires an argument") || stderr.contains("Usage:"));

    // Test missing argument for -m
    let output = Command::new(get_binary_path())
        .env("HOME", temp_dir.path())
        .arg("-m")
        // Missing message argument
        .output()
        .expect("Failed to execute binary");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success());
    assert!(stderr.contains("Option -m requires an argument") || stderr.contains("Usage:"));

    // Test missing argument for --app-token
    let output = Command::new(get_binary_path())
        .env("HOME", temp_dir.path())
        .arg("--app-token")
        // Missing token argument
        .output()
        .expect("Failed to execute binary");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success());
    assert!(
        stderr.contains("Option --app-token requires an argument") || stderr.contains("Usage:")
    );
}

#[test]
fn test_invalid_option() {
    let temp_dir = TempDir::new().unwrap();
    let _config_path = create_test_config(&temp_dir);

    let output = Command::new(get_binary_path())
        .env("HOME", temp_dir.path())
        .arg("-x") // Invalid option
        .output()
        .expect("Failed to execute binary");

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(!output.status.success());
    assert!(stderr.contains("Invalid option -x") || stderr.contains("Usage:"));
}

#[test]
fn test_unexpected_argument() {
    let temp_dir = TempDir::new().unwrap();
    let _config_path = create_test_config(&temp_dir);

    let output = Command::new(get_binary_path())
        .env("HOME", temp_dir.path())
        .arg("unexpected_arg")
        .output()
        .expect("Failed to execute binary");

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(!output.status.success());
    assert!(stderr.contains("Unexpected argument: unexpected_arg") || stderr.contains("Usage:"));
}

#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn test_network_error_with_valid_config() {
        // This test verifies that the application can load config and proceed to network stage
        // Since we have a valid config file on the system, we expect network-related errors
        let output = Command::new(get_binary_path())
            .arg("-t")
            .arg("Test Title")
            .arg("-m")
            .arg("Test Message with fake credentials")
            .output()
            .expect("Failed to execute binary");

        let stderr = String::from_utf8_lossy(&output.stderr);

        // Should fail at network stage since we're using test/invalid credentials
        assert!(!output.status.success());
        assert!(
            stderr.contains("Error sending notification")
                || stderr.contains("HTTP request failed")
                || stderr.contains("400 Bad Request")
                || stderr.contains("401")
                || stderr.contains("403")
        );
    }

    #[test]
    fn test_app_token_override_network_stage() {
        // Test that --app-token override works and reaches network stage
        let output = Command::new(get_binary_path())
            .arg("-t")
            .arg("Override Test")
            .arg("-m")
            .arg("Testing token override")
            .arg("--app-token")
            .arg("fake_override_token_12345")
            .output()
            .expect("Failed to execute binary");

        let stderr = String::from_utf8_lossy(&output.stderr);

        // Should fail at network stage with override token
        assert!(!output.status.success());
        assert!(
            stderr.contains("Error sending notification")
                || stderr.contains("HTTP request failed")
                || stderr.contains("400")
                || stderr.contains("401")
                || stderr.contains("403")
        );
    }
}

#[cfg(test)]
mod argument_parsing_tests {
    use super::*;

    #[test]
    fn test_valid_priority_values() {
        let temp_dir = TempDir::new().unwrap();
        let _config_path = create_test_config(&temp_dir);

        // Test all valid priority values
        for priority in [-2, -1, 0, 1, 2] {
            let output = Command::new(get_binary_path())
                .env("HOME", temp_dir.path())
                .arg("-t")
                .arg("Test Title")
                .arg("-m")
                .arg("Test Message")
                .arg("-p")
                .arg(priority.to_string())
                .output()
                .expect("Failed to execute binary");

            // Note: This will likely fail with network error since we're not actually
            // connecting to Pushover, but it should pass argument validation
            let stderr = String::from_utf8_lossy(&output.stderr);

            // Should not contain argument parsing errors
            assert!(!stderr.contains("Priority must be between -2 and 2"));
            assert!(!stderr.contains("Priority must be a valid integer"));
            assert!(!stderr.contains("Usage:"));
        }
    }

    #[test]
    fn test_app_token_override_parsing() {
        let temp_dir = TempDir::new().unwrap();
        let _config_path = create_test_config(&temp_dir);

        let output = Command::new(get_binary_path())
            .env("HOME", temp_dir.path())
            .arg("-t")
            .arg("Test Title")
            .arg("-m")
            .arg("Test Message")
            .arg("--app-token")
            .arg("override_token_12345")
            .output()
            .expect("Failed to execute binary");

        let stderr = String::from_utf8_lossy(&output.stderr);

        // Should not contain argument parsing errors related to --app-token
        assert!(!stderr.contains("Option --app-token requires an argument"));
        assert!(!stderr.contains("Invalid option --app-token"));
    }

    #[test]
    fn test_help_flags() {
        for help_flag in ["-h", "--help"] {
            let output = Command::new(get_binary_path())
                .arg(help_flag)
                .output()
                .expect("Failed to execute binary");

            let stderr = String::from_utf8_lossy(&output.stderr);

            // Help should be printed to stderr and program should exit
            assert!(stderr.contains("Usage:"));
            // Exit code should be 1 (as per the usage() function)
            assert_eq!(output.status.code(), Some(1));
        }
    }
}
