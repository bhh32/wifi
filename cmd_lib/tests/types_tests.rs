use cmd_lib::types::*;

#[test]
fn test_security_type_display() {
    assert_eq!(SecurityType::Open.to_string(), "Open");
    assert_eq!(SecurityType::WpaPsk.to_string(), "WPA-PSK");
    assert_eq!(SecurityType::Wpa3Sae.to_string(), "WPA3-SAE");
    assert_eq!(SecurityType::WpaEnterprise.to_string(), "WPA-Enterprise");
}

#[test]
fn test_security_type_nmcli_key_mgmt() {
    assert_eq!(SecurityType::Open.nmcli_key_mgmt(), None);
    assert_eq!(SecurityType::WpaPsk.nmcli_key_mgmt(), Some("wpa-psk"));
    assert_eq!(SecurityType::Wpa3Sae.nmcli_key_mgmt(), Some("sae"));
    assert_eq!(
        SecurityType::WpaEnterprise.nmcli_key_mgmt(),
        Some("wpa-eap")
    );
}

#[test]
fn test_connection_type_display() {
    assert_eq!(ConnectionType::Wifi.to_string(), "wifi");
    assert_eq!(ConnectionType::Ethernet.to_string(), "ethernet");
    assert_eq!(ConnectionType::Vpn.to_string(), "vpn");
}

#[test]
fn test_vpn_type_display() {
    assert_eq!(VpnType::OpenVpn.to_string(), "openvpn");
    assert_eq!(VpnType::WireGuard.to_string(), "wireguard");
    assert_eq!(VpnType::L2tp.to_string(), "l2tp");
    assert_eq!(VpnType::Pptp.to_string(), "pptp");
}

#[test]
fn test_ipv4_config_default() {
    let config = Ipv4Config::default();
    assert!(config.address.is_empty());
    assert_eq!(config.prefix, 0);
    assert!(config.gateway.is_empty());
    assert!(config.dns.is_empty());
}

#[test]
fn test_wifi_config_construction() {
    let config = WifiConfig {
        con_name: "test-wifi".to_string(),
        iface: "wlan0".to_string(),
        ssid: "MyNetwork".to_string(),
        password: Some("password123".to_string()),
        security: SecurityType::WpaPsk,
        is_hidden: false,
        auto_connect: true,
        ipv4: None,
        priority: Some(10),
    };
    assert_eq!(config.con_name, "test-wifi");
    assert_eq!(config.security, SecurityType::WpaPsk);
    assert!(config.auto_connect);
    assert_eq!(config.priority, Some(10));
}

#[test]
fn test_ethernet_config_construction() {
    let config = EthernetConfig {
        con_name: "eth-home".to_string(),
        iface: "eth0".to_string(),
        auto_connect: true,
        ipv4: Some(Ipv4Config {
            address: "192.168.1.100".to_string(),
            prefix: 24,
            gateway: "192.168.1.1".to_string(),
            dns: vec!["8.8.8.8".to_string(), "8.8.4.4".to_string()],
        }),
        priority: None,
    };
    assert_eq!(config.iface, "eth0");
    assert!(config.ipv4.is_some());
    let ipv4 = config.ipv4.unwrap();
    assert_eq!(ipv4.prefix, 24);
    assert_eq!(ipv4.dns.len(), 2);
}

#[test]
fn test_security_type_equality() {
    assert_eq!(SecurityType::WpaPsk, SecurityType::WpaPsk);
    assert_ne!(SecurityType::WpaPsk, SecurityType::Open);
}

#[test]
fn test_security_type_serialization() {
    let json = serde_json::to_string(&SecurityType::WpaPsk).unwrap();
    assert!(json.contains("WpaPsk"));
    let deserialized: SecurityType = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, SecurityType::WpaPsk);
}

#[test]
fn test_connection_info_serialization() {
    let info = ConnectionInfo {
        name: "test".to_string(),
        uuid: "abc-123".to_string(),
        conn_type: "wifi".to_string(),
        device: "wlan0".to_string(),
        active: true,
    };
    let json = serde_json::to_string(&info).unwrap();
    let deserialized: ConnectionInfo = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.name, "test");
    assert!(deserialized.active);
}

#[test]
fn test_wifi_network_serialization() {
    let net = WifiNetwork {
        ssid: "TestNet".to_string(),
        bssid: "AA:BB:CC:DD:EE:FF".to_string(),
        signal: 85,
        frequency: "2437 MHz".to_string(),
        security: "WPA2".to_string(),
        in_use: true,
    };
    let json = serde_json::to_string(&net).unwrap();
    let deserialized: WifiNetwork = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.ssid, "TestNet");
    assert_eq!(deserialized.signal, 85);
    assert!(deserialized.in_use);
}

#[test]
fn test_connection_profile_serialization() {
    let mut settings = std::collections::HashMap::new();
    settings.insert("ipv4.method".to_string(), "auto".to_string());

    let profile = ConnectionProfile {
        name: "my-wifi".to_string(),
        conn_type: ConnectionType::Wifi,
        uuid: "12345".to_string(),
        settings,
    };
    let json = serde_json::to_string_pretty(&profile).unwrap();
    let deserialized: ConnectionProfile = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.name, "my-wifi");
    assert_eq!(deserialized.conn_type, ConnectionType::Wifi);
    assert_eq!(
        deserialized.settings.get("ipv4.method"),
        Some(&"auto".to_string())
    );
}
