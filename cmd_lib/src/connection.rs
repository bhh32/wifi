use crate::error::{NetworkError, Result};
use crate::nmcli;
use crate::types::{ConnectionInfo, ConnectionProfile, ConnectionStatus, InterfaceInfo};
use crate::validate;
use std::collections::HashMap;
use tracing::info;

/// Lists all saved connections.
pub fn list() -> Result<Vec<ConnectionInfo>> {
    let output = nmcli::run(&["-t", "-f", "NAME,UUID,TYPE,DEVICE,STATE", "con", "show"])?;
    let mut connections = Vec::new();

    for line in output.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.splitn(5, ':').collect();
        if parts.len() >= 4 {
            connections.push(ConnectionInfo {
                name: parts[0].to_string(),
                uuid: parts[1].to_string(),
                conn_type: parts[2].to_string(),
                device: parts[3].to_string(),
                active: parts.get(4).map_or(false, |s| s.contains("activated")),
            });
        }
    }

    Ok(connections)
}

/// Gets the status of the current connection on a device.
pub fn status(device: Option<&str>) -> Result<Vec<ConnectionStatus>> {
    let output = nmcli::run(&[
        "-t",
        "-f",
        "DEVICE,TYPE,STATE,CONNECTION",
        "dev",
        "status",
    ])?;

    let mut statuses = Vec::new();

    for line in output.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.splitn(4, ':').collect();
        if parts.len() < 4 {
            continue;
        }

        let dev = parts[0];
        if let Some(filter_dev) = device {
            if dev != filter_dev {
                continue;
            }
        }

        let mut cs = ConnectionStatus {
            device: dev.to_string(),
            conn_type: parts[1].to_string(),
            state: parts[2].to_string(),
            connection: parts[3].to_string(),
            ip4_address: None,
            ip4_gateway: None,
            ip4_dns: Vec::new(),
            signal: None,
            ssid: None,
        };

        // Get detailed info if connected
        if cs.state.contains("connected") && !cs.connection.is_empty() && cs.connection != "--" {
            if let Ok(detail) = nmcli::run(&["-t", "-f", "all", "dev", "show", dev]) {
                for dline in detail.lines() {
                    if let Some((key, value)) = dline.split_once(':') {
                        match key.trim() {
                            "IP4.ADDRESS[1]" => cs.ip4_address = Some(value.trim().to_string()),
                            "IP4.GATEWAY" => cs.ip4_gateway = Some(value.trim().to_string()),
                            "IP4.DNS[1]" => cs.ip4_dns.push(value.trim().to_string()),
                            "IP4.DNS[2]" => cs.ip4_dns.push(value.trim().to_string()),
                            _ => {}
                        }
                    }
                }
            }

            // Get WiFi-specific info
            if cs.conn_type == "wifi" {
                if let Ok(wifi_info) =
                    nmcli::run(&["-t", "-f", "SIGNAL,SSID", "dev", "wifi", "list", "ifname", dev])
                {
                    for wline in wifi_info.lines() {
                        let wparts: Vec<&str> = wline.splitn(2, ':').collect();
                        if wparts.len() == 2 {
                            if let Ok(sig) = wparts[0].parse::<u8>() {
                                if cs.ssid.is_none() || wparts[1] == cs.connection {
                                    cs.signal = Some(sig);
                                    cs.ssid = Some(wparts[1].to_string());
                                }
                            }
                        }
                    }
                }
            }
        }

        statuses.push(cs);
    }

    Ok(statuses)
}

/// Lists available network interfaces.
pub fn list_interfaces() -> Result<Vec<InterfaceInfo>> {
    let output = nmcli::run(&["-t", "-f", "DEVICE,TYPE,STATE,CONNECTION", "dev"])?;
    let mut interfaces = Vec::new();

    for line in output.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.splitn(4, ':').collect();
        if parts.len() >= 3 {
            let connection = parts
                .get(3)
                .filter(|s| !s.is_empty() && **s != "--")
                .map(|s| s.to_string());

            interfaces.push(InterfaceInfo {
                name: parts[0].to_string(),
                iface_type: parts[1].to_string(),
                state: parts[2].to_string(),
                connection,
            });
        }
    }

    Ok(interfaces)
}

/// Detects available wireless interfaces.
pub fn detect_wifi_interfaces() -> Result<Vec<InterfaceInfo>> {
    let interfaces = list_interfaces()?;
    Ok(interfaces
        .into_iter()
        .filter(|i| i.iface_type == "wifi")
        .collect())
}

/// Detects available Ethernet interfaces.
pub fn detect_ethernet_interfaces() -> Result<Vec<InterfaceInfo>> {
    let interfaces = list_interfaces()?;
    Ok(interfaces
        .into_iter()
        .filter(|i| i.iface_type == "ethernet")
        .collect())
}

/// Removes a connection by name.
pub fn remove(con_name: &str) -> Result<()> {
    validate::connection_name(con_name)?;
    info!("Removing connection '{con_name}'");
    nmcli::run(&["con", "delete", "id", con_name])?;
    info!("Connection '{con_name}' removed");
    Ok(())
}

/// Activates a connection.
pub fn activate(con_name: &str) -> Result<()> {
    validate::connection_name(con_name)?;
    info!("Activating connection '{con_name}'");
    nmcli::run(&["con", "up", con_name])?;
    Ok(())
}

/// Deactivates a connection.
pub fn deactivate(con_name: &str) -> Result<()> {
    validate::connection_name(con_name)?;
    info!("Deactivating connection '{con_name}'");
    nmcli::run(&["con", "down", con_name])?;
    Ok(())
}

/// Exports a connection profile.
pub fn export_profile(con_name: &str) -> Result<ConnectionProfile> {
    validate::connection_name(con_name)?;
    let output = nmcli::run(&["-t", "con", "show", con_name])?;

    let mut settings = HashMap::new();
    let mut uuid = String::new();
    let mut conn_type = String::new();

    for line in output.lines() {
        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            let value = value.trim();
            match key {
                "connection.uuid" => uuid = value.to_string(),
                "connection.type" => conn_type = value.to_string(),
                _ => {
                    // Skip sensitive data
                    if !key.contains("secret") && !key.contains("psk") && !key.contains("password")
                    {
                        settings.insert(key.to_string(), value.to_string());
                    }
                }
            }
        }
    }

    if uuid.is_empty() {
        return Err(NetworkError::ConnectionNotFound(con_name.to_string()));
    }

    let conn_type_enum = match conn_type.as_str() {
        t if t.contains("wireless") || t.contains("wifi") => crate::types::ConnectionType::Wifi,
        t if t.contains("ethernet") || t.contains("802-3") => {
            crate::types::ConnectionType::Ethernet
        }
        t if t.contains("vpn") => crate::types::ConnectionType::Vpn,
        _ => {
            return Err(NetworkError::ParseError(format!(
                "Unknown connection type: {conn_type}"
            )));
        }
    };

    Ok(ConnectionProfile {
        name: con_name.to_string(),
        conn_type: conn_type_enum,
        uuid,
        settings,
    })
}

/// Imports a connection profile. Recreates the connection with stored settings.
pub fn import_profile(profile: &ConnectionProfile) -> Result<()> {
    let conn_type = match profile.conn_type {
        crate::types::ConnectionType::Wifi => "wifi",
        crate::types::ConnectionType::Ethernet => "ethernet",
        crate::types::ConnectionType::Vpn => "vpn",
    };

    // Find the interface from settings or use a default
    let iface = profile
        .settings
        .get("connection.interface-name")
        .cloned()
        .unwrap_or_else(|| "*".to_string());

    info!("Importing connection profile '{}'", profile.name);

    nmcli::run(&[
        "c",
        "add",
        "type",
        conn_type,
        "con-name",
        &profile.name,
        "ifname",
        &iface,
    ])?;

    // Apply saved settings in batches
    let applicable: Vec<(&String, &String)> = profile
        .settings
        .iter()
        .filter(|(k, _)| {
            k.starts_with("ipv4.")
                || k.starts_with("ipv6.")
                || k.starts_with("wifi.")
                || k.starts_with("connection.autoconnect")
                || k.starts_with("802-11-wireless.")
        })
        .collect();

    if !applicable.is_empty() {
        let mut args: Vec<String> = vec![
            "con".into(),
            "modify".into(),
            profile.name.clone(),
        ];
        for (key, value) in applicable {
            args.push(key.clone());
            args.push(value.clone());
        }
        nmcli::run_owned(&args)?;
    }

    info!("Connection profile '{}' imported", profile.name);
    Ok(())
}
