# Pushover Notification Tool

A fast, secure Rust implementation of a command-line tool for sending notifications via the Pushover API.

## Features

- **Pure Rust HTTPS**: Uses rustls for secure TLS connections (no curl dependency)
- **TOML Configuration**: System-wide TOML configuration file
- **Cross-platform**: Works on Linux, macOS, and Windows
- **Minimal Dependencies**: Only essential crates for TLS and TOML parsing
- **Rich Configuration**: Support for priorities, sounds, devices, and more
- **Intuitive CLI**: Simple and clear command-line interface

## Installation

### RPM Installation (Fedora/RHEL/CentOS)

Install from COPR repository:

```bash
# Enable COPR repository
sudo dnf copr enable epicgreen/pushover

# Install package
sudo dnf install pushover
```

### Quick Install (From Source)

Run the installation script:

```bash
# System-wide installation (requires root)
sudo ./install.sh

# User installation
./install.sh
```

### Manual Installation

```bash
# Build the project
cargo build --release

# Copy binary to your preferred location
sudo cp target/release/pushover /usr/local/bin/
# OR for user installation
cp target/release/pushover ~/.local/bin/

# Create configuration directory
sudo mkdir -p /etc/pushover
# OR for user installation
mkdir -p ~/.config/pushover

# Copy example configuration
sudo cp etc/pushover/config.toml /etc/pushover/
# OR for user installation
cp etc/pushover/config.toml ~/.config/pushover/
```

## Configuration

### System Configuration File

Create `/etc/pushover/config.toml` (or `~/.config/pushover/config.toml` for user installation):

```toml
[pushover]
# Your Pushover user key (required)
user = ""

# Your Pushover application token (required)
token = ""

# Default title for notifications (optional)
default_title = "Server Alert"

[notification]
# Sound (optional)
sound = "pushover"

# Device (optional)
device = "iphone"
```



### Getting Your Credentials

1. Visit [pushover.net](https://pushover.net/) and create an account
2. Your **user key** is displayed on the main dashboard
3. Create a new application at [pushover.net/apps/build](https://pushover.net/apps/build)
4. Use the **API Token/Key** from your new application

## Usage

```bash
pushover -t "Title" -m "Your message here"
```

### Command Line Options

- `-t <title>`: Notification title
- `-m <message>`: Message content (required)
- `-p <priority>`: Priority (-2 to 2, default: 0)
- `-h, --help`: Show help information

### Examples

```bash
# Basic notification
pushover -t "Server Alert" -m "Disk space is running low"

# High priority notification
pushover -t "CRITICAL" -m "Database server is down!" -p 1

# Emergency notification (requires acknowledgment)
pushover -t "EMERGENCY" -m "System failure!" -p 2

# Quiet notification
pushover -t "Info" -m "Backup completed" -p -1

# Quick test
pushover -t "Test" -m "Hello from Rust!"
```

## Configuration Options

### Notification Priorities (via -p flag)

- **-2**: No notification/alert (generates no notification)
- **-1**: Quiet notification (always sent as quiet)
- **0**: Normal priority (default when -p not specified)
- **1**: High priority (bypasses user's quiet hours)
- **2**: Emergency priority (requires acknowledgment)

### Available Sounds

`pushover` (default), `bike`, `bugle`, `cashregister`, `classical`, `cosmic`, `falling`, `gamelan`, `incoming`, `intermission`, `magic`, `mechanical`, `pianobar`, `siren`, `spacealarm`, `tugboat`, `alien`, `climb`, `persistent`, `echo`, `updown`, `none`

### Device Targeting

Specify a device name to send notifications only to that device. Use the device name as shown in your Pushover dashboard.

## Implementation Details

### Security

- Uses **rustls** for TLS 1.2/1.3 connections
- Certificate validation against Mozilla's CA bundle
- No system dependencies on OpenSSL or curl
- Memory-safe implementation in Rust

### Performance

- Fast startup time (no shell script overhead)
- Minimal memory footprint
- Direct HTTPS connections (no external process spawning)

### Dependencies

```toml
[dependencies]
rustls = "0.21"          # Pure Rust TLS implementation
webpki-roots = "0.25"    # Mozilla CA certificates
toml = "0.8"             # TOML configuration parsing
serde = "1.0"            # Serialization framework
```

## Compatibility

This implementation provides a simple, intuitive CLI interface:

```bash
pushover -t "Title" -m "Message"
```

## Installation Locations

### RPM Installation
- Binary: `/usr/bin/pushover`
- Config: `/etc/pushover/config.toml`
- Completion: `/usr/share/bash-completion/completions/pushover`
- Docs: `/usr/share/doc/pushover/`

### System Installation (root)
- Binary: `/usr/local/bin/pushover`
- Config: `/etc/pushover/config.toml`
- Completion: `/etc/bash_completion.d/pushover`

### User Installation
- Binary: `~/.local/bin/pushover`
- Config: `~/.config/pushover/config.toml`
- Completion: `~/.local/share/bash-completion/completions/pushover`

## Troubleshooting

### Configuration Issues

```bash
# Check if config file exists
ls -la /etc/pushover/config.toml

# Test configuration file
pushover -t "Test" -m "Test"
```

### Network Issues

```bash
# Test TLS connectivity
openssl s_client -connect api.pushover.net:443 -servername api.pushover.net
```

### Permission Issues

```bash
# For system installation
sudo chown root:root /usr/local/bin/pushover
sudo chmod 755 /usr/local/bin/pushover

# For user installation
chmod 755 ~/.local/bin/pushover
```

## Error Messages

- `"Error loading configuration"`: Config file missing or invalid
- `"Message is required"`: Must provide `-m` argument
- `"Priority must be between -2 and 2"`: Invalid priority value
- `"HTTP request failed"`: Network error or invalid credentials
- `"Invalid option"`: Unknown command line argument

## Uninstallation

```bash
# RPM installation
sudo dnf remove pushover

# Using the install script
sudo ./install.sh --uninstall

# Manual removal (system)
sudo rm /usr/local/bin/pushover
sudo rm -rf /etc/pushover
sudo rm /etc/bash_completion.d/pushover

# Manual removal (user)
rm ~/.local/bin/pushover
rm -rf ~/.config/pushover
rm ~/.local/share/bash-completion/completions/pushover
```

## Development

### Building from Source

```bash
git clone <repository>
cd pushover
cargo build --release
```

### Testing

```bash
# Build and test
cargo test

# Check with test config (requires config file setup)
cargo run -- -t "Test" -m "Test"
```

### Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## Packaging

The repository includes files for creating distribution packages:

- **RPM Packaging**: `pushover.spec` - RPM spec file for COPR/Fedora
- **Source Tarball**: `make-tarball.sh` - Creates source archives
- **CI/CD**: `.github/workflows/copr.yml` - Automated COPR builds

## License

This project is open source. See LICENSE file for details.

## Support

For issues related to:
- **Pushover service**: Visit [pushover.net/faq](https://pushover.net/faq)
- **This tool**: Create an issue in the repository
- **Rust installation**: Visit [rustup.rs](https://rustup.rs/)
