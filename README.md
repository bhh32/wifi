# Network Manager CLI/GUI Wrapper

Linux NetworkManager (nmcli) wrapper utilities for managing WiFi, Ethernet, and VPN connections.

## Utilities

- **wifi_cli** — Full-featured command-line interface
- **wifi** — GUI application built with libcosmic (COSMIC Desktop integration)
- **cmd_lib** — Shared core library for NetworkManager operations

## Features

### Connection Types
- **WiFi** — WPA-PSK, WPA3-SAE, WPA-Enterprise, and open networks
- **Ethernet** — Wired connections with DHCP or manual IP
- **VPN** — OpenVPN, WireGuard, L2TP, PPTP

### Core Functionality
- Add, remove, edit, activate, and deactivate connections
- Scan for available WiFi networks
- List saved connections and their status
- Auto-detect network interfaces
- Manual IPv4 configuration (address, netmask, gateway, DNS)
- Connection priority management
- WiFi hotspot/AP mode
- Connection profile import/export (JSON)
- Input validation and sanitization
- Structured error handling (no panics)
- Logging via `tracing` crate

### CLI Commands
```
wifi_cli add          # Add WiFi connection
wifi_cli add-ethernet # Add Ethernet connection
wifi_cli add-vpn      # Add VPN connection
wifi_cli remove       # Remove a connection
wifi_cli edit         # Edit connection properties
wifi_cli scan         # Scan for WiFi networks
wifi_cli list         # List saved connections
wifi_cli status       # Show connection status
wifi_cli interfaces   # List network interfaces
wifi_cli hotspot      # Create WiFi hotspot
wifi_cli up           # Activate a connection
wifi_cli down         # Deactivate a connection
wifi_cli export       # Export connection profile
wifi_cli import       # Import connection profile
```

Use `wifi_cli --help` or `wifi_cli <command> --help` for detailed usage.

### GUI
The GUI application provides a tabbed interface for:
- **WiFi tab** — Add WiFi connections with network scanning, security selection, DHCP/manual IP
- **Ethernet tab** — Add Ethernet connections with interface auto-detection
- **Connections tab** — View, activate, deactivate, and remove saved connections

Built with [libcosmic](https://github.com/pop-os/libcosmic) for COSMIC Desktop Environment integration.

## Building

### Prerequisites
- Rust toolchain (edition 2021+)
- Linux with NetworkManager
- System packages: `libxkbcommon-dev libwayland-dev pkg-config cmake libexpat1-dev libfontconfig-dev libfreetype-dev`

### Build
```bash
cargo build --release
```

Binaries will be in `target/release/wifi_cli` and `target/release/wifi`.

### Test
```bash
cargo test --workspace
```

## Packaging

Build scripts are provided for multiple package formats:

```bash
# Debian/Ubuntu
./packaging/build-deb.sh

# RPM (Fedora/RHEL)
./packaging/build-rpm.sh

# Arch Linux (AUR)
# Use packaging/PKGBUILD
```

## Architecture

```
wifi/
├── cmd_lib/          # Core library — nmcli wrapper with validation, error handling
│   ├── wifi.rs       # WiFi operations (setup, scan, edit, hotspot)
│   ├── ethernet.rs   # Ethernet operations
│   ├── vpn.rs        # VPN operations (OpenVPN, WireGuard, L2TP, PPTP)
│   ├── connection.rs # Connection management (list, status, activate, export/import)
│   ├── validate.rs   # Input validation and sanitization
│   ├── error.rs      # Error types
│   ├── types.rs      # Shared data types
│   ├── nmcli.rs      # nmcli command execution helper
│   └── async_nmcli.rs # Async command execution (tokio, feature-gated)
├── wifi_cli/         # CLI application (clap + rpassword)
├── wifi/             # GUI application (libcosmic)
├── packaging/        # Build scripts for deb, rpm, AUR
└── .github/          # CI/CD workflows
```

## License

GPL-3.0
