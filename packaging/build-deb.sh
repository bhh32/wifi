#!/bin/bash
set -e

VERSION="0.3.0"
PKG_NAME="wifi-manager"
BUILD_DIR="target/deb-build"

echo "Building ${PKG_NAME} v${VERSION} .deb package..."

# Build release binaries
cargo build --release

# Create package structure
rm -rf "${BUILD_DIR}"
mkdir -p "${BUILD_DIR}/DEBIAN"
mkdir -p "${BUILD_DIR}/usr/bin"
mkdir -p "${BUILD_DIR}/usr/share/applications"

# Copy control file
cp packaging/deb/control "${BUILD_DIR}/DEBIAN/control"

# Copy binaries
cp target/release/wifi_cli "${BUILD_DIR}/usr/bin/wifi-cli"
cp target/release/wifi "${BUILD_DIR}/usr/bin/wifi-manager-gui"

# Create desktop entry for GUI
cat > "${BUILD_DIR}/usr/share/applications/wifi-manager.desktop" << EOF
[Desktop Entry]
Name=WiFi Manager
Comment=Network Manager GUI Utility
Exec=wifi-manager-gui
Terminal=false
Type=Application
Categories=System;Network;Settings;
Keywords=wifi;network;connection;
EOF

# Build the package
dpkg-deb --build "${BUILD_DIR}" "target/${PKG_NAME}_${VERSION}_amd64.deb"

echo "Package built: target/${PKG_NAME}_${VERSION}_amd64.deb"
