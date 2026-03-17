#!/bin/bash
set -e

VERSION="0.3.0"
PKG_NAME="wifi-manager"

echo "Building ${PKG_NAME} v${VERSION} RPM package..."

# Build release binaries
cargo build --release

# Create rpmbuild structure
RPM_DIR="${HOME}/rpmbuild"
mkdir -p "${RPM_DIR}"/{BUILD,RPMS,SOURCES,SPECS,SRPMS}

# Create spec file
cat > "${RPM_DIR}/SPECS/${PKG_NAME}.spec" << EOF
Name:           ${PKG_NAME}
Version:        ${VERSION}
Release:        1%{?dist}
Summary:        Network Manager CLI/GUI Wrapper

License:        GPL-3.0
URL:            https://github.com/bhh32/wifi

Requires:       NetworkManager

%description
A set of utilities for managing WiFi, Ethernet, and VPN connections
via Linux NetworkManager. Includes both a CLI tool and GUI application.

%install
mkdir -p %{buildroot}%{_bindir}
mkdir -p %{buildroot}%{_datadir}/applications
install -m 755 %{_topdir}/../target/release/wifi_cli %{buildroot}%{_bindir}/wifi-cli
install -m 755 %{_topdir}/../target/release/wifi %{buildroot}%{_bindir}/wifi-manager-gui

cat > %{buildroot}%{_datadir}/applications/${PKG_NAME}.desktop << DESKTOP
[Desktop Entry]
Name=WiFi Manager
Comment=Network Manager GUI Utility
Exec=wifi-manager-gui
Terminal=false
Type=Application
Categories=System;Network;Settings;
Keywords=wifi;network;connection;
DESKTOP

%files
%{_bindir}/wifi-cli
%{_bindir}/wifi-manager-gui
%{_datadir}/applications/${PKG_NAME}.desktop
EOF

# Build the RPM
rpmbuild -bb "${RPM_DIR}/SPECS/${PKG_NAME}.spec"

echo "RPM package built in ${RPM_DIR}/RPMS/"
