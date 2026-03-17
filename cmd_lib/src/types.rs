use serde::{Deserialize, Serialize};
use std::fmt;

/// Security type for a WiFi connection.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SecurityType {
    /// No security (open network)
    Open,
    /// WPA/WPA2 Pre-Shared Key
    WpaPsk,
    /// WPA3 SAE (Simultaneous Authentication of Equals)
    Wpa3Sae,
    /// WPA Enterprise (802.1X)
    WpaEnterprise,
}

impl fmt::Display for SecurityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Open => write!(f, "Open"),
            Self::WpaPsk => write!(f, "WPA-PSK"),
            Self::Wpa3Sae => write!(f, "WPA3-SAE"),
            Self::WpaEnterprise => write!(f, "WPA-Enterprise"),
        }
    }
}

impl SecurityType {
    /// Returns the nmcli key-mgmt value for this security type.
    pub fn nmcli_key_mgmt(&self) -> Option<&str> {
        match self {
            Self::Open => None,
            Self::WpaPsk => Some("wpa-psk"),
            Self::Wpa3Sae => Some("sae"),
            Self::WpaEnterprise => Some("wpa-eap"),
        }
    }
}

/// Connection type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConnectionType {
    Wifi,
    Ethernet,
    Vpn,
}

impl fmt::Display for ConnectionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Wifi => write!(f, "wifi"),
            Self::Ethernet => write!(f, "ethernet"),
            Self::Vpn => write!(f, "vpn"),
        }
    }
}

/// Manual IPv4 configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ipv4Config {
    pub address: String,
    pub prefix: u32,
    pub gateway: String,
    pub dns: Vec<String>,
}

/// Configuration for creating a WiFi connection.
#[derive(Debug, Clone)]
pub struct WifiConfig {
    pub con_name: String,
    pub iface: String,
    pub ssid: String,
    pub password: Option<String>,
    pub security: SecurityType,
    pub is_hidden: bool,
    pub auto_connect: bool,
    pub ipv4: Option<Ipv4Config>,
    pub priority: Option<i32>,
}

/// Configuration for creating an Ethernet connection.
#[derive(Debug, Clone)]
pub struct EthernetConfig {
    pub con_name: String,
    pub iface: String,
    pub auto_connect: bool,
    pub ipv4: Option<Ipv4Config>,
    pub priority: Option<i32>,
}

/// Configuration for creating a VPN connection.
#[derive(Debug, Clone)]
pub struct VpnConfig {
    pub con_name: String,
    pub vpn_type: VpnType,
    pub gateway: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub config_file: Option<String>,
    pub auto_connect: bool,
}

/// Supported VPN types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VpnType {
    OpenVpn,
    WireGuard,
    L2tp,
    Pptp,
}

impl fmt::Display for VpnType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OpenVpn => write!(f, "openvpn"),
            Self::WireGuard => write!(f, "wireguard"),
            Self::L2tp => write!(f, "l2tp"),
            Self::Pptp => write!(f, "pptp"),
        }
    }
}

/// Information about an available WiFi network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WifiNetwork {
    pub ssid: String,
    pub bssid: String,
    pub signal: u8,
    pub frequency: String,
    pub security: String,
    pub in_use: bool,
}

/// Information about a saved connection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionInfo {
    pub name: String,
    pub uuid: String,
    pub conn_type: String,
    pub device: String,
    pub active: bool,
}

/// Status of the current network connection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionStatus {
    pub device: String,
    pub conn_type: String,
    pub state: String,
    pub connection: String,
    pub ip4_address: Option<String>,
    pub ip4_gateway: Option<String>,
    pub ip4_dns: Vec<String>,
    pub signal: Option<u8>,
    pub ssid: Option<String>,
}

/// Information about a network interface.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceInfo {
    pub name: String,
    pub iface_type: String,
    pub state: String,
    pub connection: Option<String>,
}

/// Hotspot configuration.
#[derive(Debug, Clone)]
pub struct HotspotConfig {
    pub iface: String,
    pub ssid: String,
    pub password: Option<String>,
    pub band: Option<String>,
    pub channel: Option<u32>,
}

/// A connection profile for import/export.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionProfile {
    pub name: String,
    pub conn_type: ConnectionType,
    pub uuid: String,
    pub settings: std::collections::HashMap<String, String>,
}
