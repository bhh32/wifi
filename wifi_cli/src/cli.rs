use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(
    name = "wifi_cli",
    version,
    about = "Network Manager CLI wrapper for WiFi, Ethernet, and VPN connections",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose logging output
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Add a new WiFi connection
    #[command(alias = "add-wifi")]
    Add {
        #[arg(short, long, required = true, help = "Connection display name")]
        con_name: String,
        #[arg(short, long, help = "Wireless interface name (auto-detected if omitted)")]
        iface: Option<String>,
        #[arg(short, long, required = true, help = "Network SSID")]
        ssid: String,
        #[arg(long, help = "This is a hidden network")]
        hidden: bool,
        #[arg(short, long, help = "Automatically connect")]
        auto_con: bool,
        #[arg(long, value_enum, default_value_t = SecurityArg::WpaPsk, help = "Security type")]
        security: SecurityArg,
        #[arg(long, help = "IPv4 address (e.g., 192.168.1.100/24)")]
        ipv4: Option<String>,
        #[arg(long, help = "Gateway address")]
        gateway: Option<String>,
        #[arg(long, help = "DNS server(s), comma-separated")]
        dns: Option<String>,
        #[arg(long, help = "Connection priority (-999 to 999)")]
        priority: Option<i32>,
    },

    /// Add a new Ethernet connection
    #[command(alias = "add-eth")]
    AddEthernet {
        #[arg(short, long, required = true, help = "Connection display name")]
        con_name: String,
        #[arg(short, long, help = "Ethernet interface name (auto-detected if omitted)")]
        iface: Option<String>,
        #[arg(short, long, help = "Automatically connect")]
        auto_con: bool,
        #[arg(long, help = "IPv4 address (e.g., 192.168.1.100/24)")]
        ipv4: Option<String>,
        #[arg(long, help = "Gateway address")]
        gateway: Option<String>,
        #[arg(long, help = "DNS server(s), comma-separated")]
        dns: Option<String>,
        #[arg(long, help = "Connection priority (-999 to 999)")]
        priority: Option<i32>,
    },

    /// Add a new VPN connection
    #[command(alias = "add-vpn")]
    AddVpn {
        #[arg(short, long, required = true, help = "Connection display name")]
        con_name: String,
        #[arg(long, value_enum, required = true, help = "VPN type")]
        vpn_type: VpnTypeArg,
        #[arg(short, long, required = true, help = "VPN gateway/server address")]
        gateway: String,
        #[arg(short, long, help = "Username")]
        username: Option<String>,
        #[arg(short, long, help = "Configuration file path (required for WireGuard)")]
        config_file: Option<String>,
        #[arg(short, long, help = "Automatically connect")]
        auto_con: bool,
    },

    /// Remove a network connection
    Remove {
        #[arg(short, long, required = true, help = "Connection display name")]
        con_name: String,
    },

    /// Edit an existing connection
    Edit {
        #[arg(short, long, required = true, help = "Connection name to edit")]
        con_name: String,
        #[arg(long, help = "Set auto-connect")]
        auto_con: Option<bool>,
        #[arg(long, help = "Set connection priority (-999 to 999)")]
        priority: Option<i32>,
        #[arg(long, help = "Set IPv4 address (e.g., 192.168.1.100/24)")]
        ipv4: Option<String>,
        #[arg(long, help = "Set gateway address")]
        gateway: Option<String>,
        #[arg(long, help = "Set DNS server(s), comma-separated")]
        dns: Option<String>,
    },

    /// Scan for available WiFi networks
    Scan {
        #[arg(short, long, help = "Interface to scan on")]
        iface: Option<String>,
    },

    /// List saved connections
    List,

    /// Show connection status
    Status {
        #[arg(short, long, help = "Show status for specific device")]
        device: Option<String>,
    },

    /// List network interfaces
    Interfaces,

    /// Create a WiFi hotspot
    Hotspot {
        #[arg(short, long, required = true, help = "Interface to use")]
        iface: String,
        #[arg(short, long, required = true, help = "Hotspot SSID")]
        ssid: String,
        #[arg(short, long, help = "Hotspot password")]
        password: Option<String>,
        #[arg(short, long, help = "WiFi band (a or bg)")]
        band: Option<String>,
        #[arg(short, long, help = "WiFi channel")]
        channel: Option<u32>,
    },

    /// Activate a saved connection
    Up {
        #[arg(short, long, required = true, help = "Connection name")]
        con_name: String,
    },

    /// Deactivate a connection
    Down {
        #[arg(short, long, required = true, help = "Connection name")]
        con_name: String,
    },

    /// Export a connection profile to JSON
    Export {
        #[arg(short, long, required = true, help = "Connection name")]
        con_name: String,
        #[arg(short, long, help = "Output file path (stdout if omitted)")]
        output: Option<String>,
    },

    /// Import a connection profile from JSON
    Import {
        #[arg(short, long, required = true, help = "Input JSON file path")]
        file: String,
    },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum SecurityArg {
    Open,
    WpaPsk,
    Wpa3Sae,
    WpaEnterprise,
}

impl From<SecurityArg> for cmd_lib::SecurityType {
    fn from(val: SecurityArg) -> Self {
        match val {
            SecurityArg::Open => cmd_lib::SecurityType::Open,
            SecurityArg::WpaPsk => cmd_lib::SecurityType::WpaPsk,
            SecurityArg::Wpa3Sae => cmd_lib::SecurityType::Wpa3Sae,
            SecurityArg::WpaEnterprise => cmd_lib::SecurityType::WpaEnterprise,
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum VpnTypeArg {
    Openvpn,
    Wireguard,
    L2tp,
    Pptp,
}

impl From<VpnTypeArg> for cmd_lib::VpnType {
    fn from(val: VpnTypeArg) -> Self {
        match val {
            VpnTypeArg::Openvpn => cmd_lib::VpnType::OpenVpn,
            VpnTypeArg::Wireguard => cmd_lib::VpnType::WireGuard,
            VpnTypeArg::L2tp => cmd_lib::VpnType::L2tp,
            VpnTypeArg::Pptp => cmd_lib::VpnType::Pptp,
        }
    }
}
