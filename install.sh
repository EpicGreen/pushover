#!/usr/bin/env bash

# Pushover Installation Script
# This script installs the pushover binary and sets up configuration

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print colored output
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if running as root for system installation
check_root() {
    if [[ $EUID -eq 0 ]]; then
        INSTALL_DIR="/usr/local/bin"
        CONFIG_DIR="/etc/pushover"
        print_info "Installing system-wide (requires root)"
    else
        INSTALL_DIR="$HOME/.local/bin"
        CONFIG_DIR="$HOME/.config/pushover"
        print_info "Installing for current user only"
        mkdir -p "$INSTALL_DIR"
    fi
}

# Build the project
build_project() {
    print_info "Building pushover..."
    if ! command -v cargo &> /dev/null; then
        print_error "Rust/Cargo not found. Please install Rust from https://rustup.rs/"
        exit 1
    fi

    cargo build --release
    if [[ $? -ne 0 ]]; then
        print_error "Build failed"
        exit 1
    fi
    print_success "Build completed"
}

# Install binary
install_binary() {
    print_info "Installing binary to $INSTALL_DIR..."
    cp target/release/pushover "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/pushover"
    print_success "Binary installed to $INSTALL_DIR/pushover"
}

# Setup configuration
setup_config() {
    print_info "Setting up configuration directory..."
    mkdir -p "$CONFIG_DIR"

    if [[ ! -f "$CONFIG_DIR/config.toml" ]]; then
        print_info "Installing example configuration..."
        cp etc/pushover/config.toml "$CONFIG_DIR/"
        print_warning "Configuration template installed to $CONFIG_DIR/config.toml"
        print_warning "Please edit this file with your actual Pushover credentials"
    else
        print_info "Configuration already exists at $CONFIG_DIR/config.toml"
    fi
}

# Setup shell completion (optional)
setup_completion() {
    if [[ $EUID -eq 0 ]]; then
        COMPLETION_DIR="/etc/bash_completion.d"
    else
        COMPLETION_DIR="$HOME/.local/share/bash-completion/completions"
        mkdir -p "$COMPLETION_DIR"
    fi

    # Check if bash completion file exists
    if [[ ! -f "etc/bash-completion/pushover" ]]; then
        print_warning "Bash completion file not found at etc/bash-completion/pushover"
        return 0
    fi

    # Install bash completion from file
    print_info "Installing bash completion..."
    cp etc/bash-completion/pushover "$COMPLETION_DIR/"
    chmod 644 "$COMPLETION_DIR/pushover"

    print_success "Bash completion installed to $COMPLETION_DIR/pushover"
}

# Check PATH
check_path() {
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        print_warning "$INSTALL_DIR is not in your PATH"
        if [[ $EUID -ne 0 ]]; then
            print_info "Add this to your ~/.bashrc or ~/.zshrc:"
            echo "    export PATH=\"\$PATH:$INSTALL_DIR\""
        fi
    fi
}

# Show post-installation instructions
show_instructions() {
    echo
    print_success "Installation completed!"
    echo
    print_info "Next steps:"
    echo "1. Edit the configuration file: $CONFIG_DIR/config.toml"
    echo "2. Add your Pushover user key and application token"
    echo "3. Test the installation:"
    echo "   pushover -t \"Test\" -m \"Hello from pushover!\""
    echo
    print_info "For help: pushover --help"
    echo
    print_info "Get your credentials at: https://pushover.net/"
}

# Main installation process
main() {
    print_info "Pushover Installation Script"
    echo

    # Check if we're in the right directory
    if [[ ! -f "Cargo.toml" ]] || [[ ! -f "src/main.rs" ]]; then
        print_error "Please run this script from the pushover project directory"
        exit 1
    fi

    check_root
    build_project
    install_binary
    setup_config
    setup_completion
    check_path
    show_instructions
}

# Handle command line arguments
case "${1:-}" in
    --help|-h)
        echo "Pushover Installation Script"
        echo
        echo "Usage: $0 [options]"
        echo
        echo "Options:"
        echo "  --help, -h     Show this help message"
        echo "  --uninstall    Remove pushover installation"
        echo
        echo "This script will:"
        echo "- Build the pushover binary"
        echo "- Install it to the appropriate location"
        echo "- Set up configuration directory"
        echo "- Install bash completion"
        echo
        echo "Root installation: /usr/local/bin and /etc/pushover"
        echo "User installation: ~/.local/bin and ~/.config/pushover"
        exit 0
        ;;
    --uninstall)
        check_root
        print_info "Uninstalling pushover..."
        rm -f "$INSTALL_DIR/pushover"
        rm -rf "$CONFIG_DIR"
        if [[ $EUID -eq 0 ]]; then
            rm -f "/etc/bash_completion.d/pushover"
        else
            rm -f "$HOME/.local/share/bash-completion/completions/pushover"
        fi
        print_success "Pushover uninstalled"
        exit 0
        ;;
    "")
        main
        ;;
    *)
        print_error "Unknown option: $1"
        echo "Use --help for usage information"
        exit 1
        ;;
esac
