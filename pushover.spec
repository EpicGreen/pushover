Name:           pushover
Version:        0.1.0
Release:        1%{?dist}
Summary:        A secure command-line tool for sending Pushover notifications

License:        AGPL-3.0-or-later
URL:            https://github.com/epicgreen/pushover
Source0:        https://github.com/epicgreen/pushover/archive/v%{version}/%{name}-%{version}.tar.gz

BuildRequires:  rust >= 1.70
BuildRequires:  cargo
BuildRequires:  gcc
BuildRequires:  openssl-devel

Requires:       glibc

# Disable debuginfo package generation
%global debug_package %{nil}

# Don't strip the binary to preserve Rust symbols
%global __os_install_post %{nil}

%description
A fast, secure Rust implementation of a command-line tool for sending
notifications via the Pushover API. Features pure Rust HTTPS implementation
using rustls, TOML configuration, and support for all Pushover notification
options including priorities, sounds, and device targeting.

%prep
%autosetup

%build
# Set up cargo home in build directory
export CARGO_HOME=$PWD/.cargo
# Build with verbose output and offline mode disabled
cargo build --release --verbose

%install
# Install binary
install -D -m 755 target/release/%{name} %{buildroot}%{_bindir}/%{name}

# Install configuration directory and example config
install -d %{buildroot}%{_sysconfdir}/%{name}
install -D -m 644 etc/%{name}/config.toml %{buildroot}%{_sysconfdir}/%{name}/config.toml

# Install bash completion
install -D -m 644 /dev/stdin %{buildroot}%{_datadir}/bash-completion/completions/%{name} << 'EOF'
# Bash completion for pushover
_pushover() {
    local cur prev opts
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    opts="-t -m -p -h --help"

    case ${prev} in
        -t)
            # No completion for title
            return 0
            ;;
        -m)
            # No completion for message
            return 0
            ;;
        -p)
            # Suggest priority values
            COMPREPLY=( $(compgen -W "-2 -1 0 1 2" -- ${cur}) )
            return 0
            ;;
        *)
            ;;
    esac

    COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
    return 0
}
complete -F _pushover pushover
EOF

# Install documentation
install -d %{buildroot}%{_docdir}/%{name}
install -m 644 README.md %{buildroot}%{_docdir}/%{name}/
install -m 644 CHANGELOG.md %{buildroot}%{_docdir}/%{name}/

%files
%license LICENSE
%doc %{_docdir}/%{name}/README.md
%doc %{_docdir}/%{name}/CHANGELOG.md
%{_bindir}/%{name}
%config(noreplace) %{_sysconfdir}/%{name}/config.toml
%{_datadir}/bash-completion/completions/%{name}

%post
echo "Pushover has been installed!"
echo "Configure your credentials in /etc/pushover/config.toml"
echo "Get your credentials at: https://pushover.net/"
echo ""
echo "Example usage:"
echo "  pushover -t \"Alert\" -m \"Your message here\""
echo "  pushover -t \"Critical\" -m \"High priority alert\" -p 1"

%changelog
* Thu Dec 19 2024 Package Maintainer <maintainer@example.com> - 0.1.0-1
- Initial release for COPR
- Pure Rust HTTPS implementation using rustls
- TOML configuration system
- Command-line priority control (-p flag)
- Support for notification sounds and device targeting
- Bash completion support
- Cross-platform compatibility
- No external dependencies beyond standard libraries
