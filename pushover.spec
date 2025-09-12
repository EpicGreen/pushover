%global commit %{?commitish}%{!?commitish:HEAD}
%global shortcommit %(c=%{commit}; echo ${c:0:7})
%global commit_date %(date +%%Y%%m%%d)

Name:           pushover
Version:        0.1.3
Release:        %{commit_date}%{shortcommit}%{?dist}
Summary:        A secure command-line tool for sending Pushover notifications

License:        AGPL-3.0-or-later
URL:            https://github.com/epicgreen/pushover
Source0:        https://github.com/epicgreen/pushover/archive/%{commit}/%{name}-%{commit}.tar.gz

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
mv %{name}-* %{name}-%{version}
cd %{name}-%{version}

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
install -D -m 644 etc/bash-completion/%{name} %{buildroot}%{_datadir}/bash-completion/completions/%{name}

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

%changelog
* Wed Sep 3 2025 Ante de Baas <packages@debaas.net> - 0.1.3
- Pure Rust HTTPS implementation using rustls
- TOML configuration system
- Command-line priority control (-p flag)
- Support for notification sounds and device targeting
- Bash completion support
- Cross-platform compatibility
- No external dependencies beyond standard libraries
