use cmd_lib::types::{ConnectionInfo, InterfaceInfo, WifiNetwork};

#[derive(Debug, Clone)]
pub enum Message {
    // Tab navigation
    SetTab(Tab),

    // WiFi form fields
    UpdateConName(String),
    UpdateSsid(String),
    UpdatePsk(String),
    UpdateIPv4(String),
    UpdateNetmask(String),
    UpdateGateway(String),
    UpdateDns(String),
    ToggleHidden(bool),
    ToggleAutoCon(bool),
    ToggleDHCP(bool),
    SetSecurity(SecurityChoice),
    SetPriority(String),

    // Ethernet form fields
    UpdateEthConName(String),
    UpdateEthIPv4(String),
    UpdateEthNetmask(String),
    UpdateEthGateway(String),
    UpdateEthDns(String),
    ToggleEthAutoCon(bool),
    ToggleEthDHCP(bool),

    // Interface selection
    SelectWifiInterface(usize),
    SelectEthInterface(usize),

    // Actions
    AddWifi,
    AddEthernet,
    RemoveConnection,
    ScanNetworks,
    RefreshConnections,
    RefreshInterfaces,
    ActivateConnection(String),
    DeactivateConnection(String),
    SelectNetwork(WifiNetwork),

    // Results from async operations
    ScanComplete(Vec<WifiNetwork>),
    ConnectionsLoaded(Vec<ConnectionInfo>),
    InterfacesLoaded(Vec<InterfaceInfo>),
    OperationSuccess(String),
    OperationError(String),

    // Connection selection for removal
    SelectConnection(usize),

    // Dismiss status messages
    DismissStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Wifi,
    Ethernet,
    Connections,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityChoice {
    Open,
    WpaPsk,
    Wpa3Sae,
    WpaEnterprise,
}

impl std::fmt::Display for SecurityChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open => write!(f, "Open (No Security)"),
            Self::WpaPsk => write!(f, "WPA/WPA2-PSK"),
            Self::Wpa3Sae => write!(f, "WPA3-SAE"),
            Self::WpaEnterprise => write!(f, "WPA-Enterprise"),
        }
    }
}

impl From<SecurityChoice> for cmd_lib::SecurityType {
    fn from(val: SecurityChoice) -> Self {
        match val {
            SecurityChoice::Open => Self::Open,
            SecurityChoice::WpaPsk => Self::WpaPsk,
            SecurityChoice::Wpa3Sae => Self::Wpa3Sae,
            SecurityChoice::WpaEnterprise => Self::WpaEnterprise,
        }
    }
}
