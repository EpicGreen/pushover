Name:           pushover
Version:        0.2.1
Release:        1%{?dist}
Summary:        A secure command-line tool for sending Pushover notifications

License:        AGPL-3.0-or-later
URL:            https://github.com/epicgreen/%{name}
Source0:        https://github.com/epicgreen/%{name}/archive/refs/tags/v%{version}.tar.gz

BuildRequires:  rust >= 1.70
BuildRequires:  cargo
BuildRequires:  gcc
BuildRequires:  openssl-devel
BuildRequires:  git

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
%setup -q -c

%build
cd %{name}-%{version}
export CARGO_HOME=$PWD/.cargo
cargo build --release --verbose

%install
cd %{name}-%{version}
install -D -m 755 target/release/%{name} %{buildroot}%{_bindir}/%{name}

# Install configuration directory and example config
install -d %{buildroot}%{_sysconfdir}/%{name}
install -D -m 644 etc/%{name}/config.toml %{buildroot}%{_sysconfdir}/%{name}/config.toml

# Install bash completion
install -D -m 644 etc/bash-completion/%{name} %{buildroot}%{_datadir}/bash-completion/completions/%{name}

# Install license
install -d %{buildroot}%{_licensedir}/%{name}
install -m 644 LICENSE %{buildroot}%{_licensedir}/%{name}/LICENSE

# Install documentation
install -d %{buildroot}%{_docdir}/%{name}
install -m 644 README.md %{buildroot}%{_docdir}/%{name}/
install -m 644 CHANGELOG.md %{buildroot}%{_docdir}/%{name}/

%files
%license %{_licensedir}/%{name}/LICENSE
%doc %{_docdir}/%{name}/README.md
%doc %{_docdir}/%{name}/CHANGELOG.md
%{_bindir}/%{name}
%config(noreplace) %{_sysconfdir}/%{name}/config.toml
%{_datadir}/bash-completion/completions/%{name}

%changelog
* Wed Oct 2 2024 Ante de Baas <packages@debaas.net> - 0.2.0
- Add tests
- Allow for app_token override via commandline

* Wed Sep 3 2025 Ante de Baas <packages@debaas.net> - 0.1.3
- Pure Rust HTTPS implementation using rustls
- TOML configuration system
- Command-line priority control (-p flag)
- Support for notification sounds and device targeting
- Bash completion support
- Cross-platform compatibility
- No external dependencies beyond standard libraries
