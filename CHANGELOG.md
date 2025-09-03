# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2024-12-19

### 🎉 Initial Release

A modern, secure command-line tool for sending notifications via the Pushover API, built in Rust.

### ✨ Features

- **Pure Rust HTTPS Implementation**
  - Uses `rustls` for TLS 1.2/1.3 connections
  - No dependency on system curl or OpenSSL
  - Certificate validation against Mozilla's CA bundle
  - Memory-safe implementation

- **TOML Configuration System**
  - System-wide configuration at `/etc/pushover/config.toml`
  - User-specific configuration at `~/.config/pushover/config.toml`
  - Simple, declarative configuration format

- **Rich Notification Options**
  - Command-line priority control (-p flag, -2 to 2)
  - Multiple sound options (pushover, siren, alien, etc.)
  - Device targeting support
  - Custom default titles

- **Professional CLI Experience**
  - Intuitive command-line interface with priority control
  - Bash completion support
  - Comprehensive help messages
  - Clear error reporting and validation

- **Easy Installation**
  - RPM packages for Fedora/RHEL/CentOS via COPR
  - Automated installation script (`install.sh`)
  - Support for both system-wide and user installations
  - Uninstallation support
  - Bash completion setup

### 🔧 Technical Features

- **Performance**: Fast startup and execution
- **Security**: Modern TLS implementation with certificate validation
- **Memory Safety**: Rust's memory safety guarantees
- **Cross-Platform**: Works on Linux, macOS, and Windows
- **Minimal Dependencies**: Only essential crates for TLS and TOML

### 📦 Dependencies

- `rustls 0.21` - Pure Rust TLS implementation
- `webpki-roots 0.25` - Mozilla CA certificate bundle
- `toml 0.8` - TOML configuration parser
- `serde 1.0` - Serialization framework

### 🚀 Installation

#### RPM Installation
```bash
# Enable COPR repository
sudo dnf copr enable epicgreen/pushover

# Install package
sudo dnf install pushover
```

#### Quick Install from Source
```bash
# System-wide installation (requires root)
sudo ./install.sh

# User installation
./install.sh
```

#### Manual Installation
```bash
cargo build --release
sudo cp target/release/pushover /usr/local/bin/
sudo mkdir -p /etc/pushover
sudo cp etc/pushover/config.toml /etc/pushover/
```

### ⚙️ Configuration

Create `/etc/pushover/config.toml`:

```toml
[pushover]
user = "your_user_key"
token = "your_app_token"
default_title = "Server Alert"

[notification]
sound = "pushover"
device = "iphone"
```

### 📖 Usage

```bash
# Basic notification
pushover -t "Server Alert" -m "Disk space low"

# High priority notification
pushover -t "CRITICAL" -m "Database server is down!" -p 1

# Emergency notification (requires acknowledgment)
pushover -t "EMERGENCY" -m "System failure!" -p 2

# Quiet notification
pushover -t "Info" -m "Backup completed" -p -1

# Quick test
pushover -t "Test" -m "Hello from Rust!"

# Show help
pushover --help
```

### 🎯 Key Benefits

- **Secure**: Modern TLS implementation, no shell vulnerabilities
- **Fast**: Direct HTTPS connections, no external process overhead
- **Reliable**: Memory-safe Rust implementation
- **Simple**: Single configuration file, clear CLI interface
- **Professional**: Proper installation, completion, documentation

### 📁 Project Structure

```
pushover/
├── src/
│   └── main.rs              # Main Rust application
├── etc/
│   └── pushover/
│       └── config.toml      # Example configuration
├── install.sh               # Installation script
├── make-tarball.sh          # RPM source tarball generator
├── pushover.spec            # RPM spec fil
├── Cargo.toml              # Rust dependencies
├── README.md               # Comprehensive documentation
└── CHANGELOG.md            # This file
```

### 🔒 Security

- Modern TLS 1.2/1.3 support only
- Certificate validation against Mozilla CA bundle
- No shell command injection vulnerabilities
- Memory-safe Rust implementation

### 🎛️ Configuration Options

#### Notification Priorities
- **-2**: No notification/alert
- **-1**: Quiet notification
- **0**: Normal priority (default)
- **1**: High priority (bypasses quiet hours)
- **2**: Emergency priority (requires acknowledgment)

#### Available Sounds
`pushover`, `bike`, `bugle`, `cashregister`, `classical`, `cosmic`, `falling`, `gamelan`, `incoming`, `intermission`, `magic`, `mechanical`, `pianobar`, `siren`, `spacealarm`, `tugboat`, `alien`, `climb`, `persistent`, `echo`, `updown`, `none`

### 📚 Documentation

- Complete README with installation and usage instructions
- Inline code documentation
- Example configuration files
- Troubleshooting guide
- Installation script with help

### 🐛 Known Issues

None at this time.

### 📦 Packaging

- **RPM**: Available via COPR for Fedora, RHEL, CentOS
- **Source**: Manual compilation with Cargo
- **Portable**: Single binary deployment option

### 🔮 Future Enhancements

- Configuration validation with helpful error messages
- Support for message attachments
- Retry logic for failed deliveries
- Integration with system notification services

---

## Development

### Building from Source

```bash
git clone <repository>
cd pushover
cargo build --release
```

### Requirements

- Rust 1.70+ (2024 edition)
- Valid Pushover user key and application token

### Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

### License

This project is open source. See LICENSE file for details.
