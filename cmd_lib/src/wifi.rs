use crate::error::{NetworkError, Result};
use crate::nmcli;
use crate::types::{HotspotConfig, SecurityType, WifiConfig, WifiNetwork};
use crate::validate;
use tracing::info;

/// Sets up a new WiFi network connection using NetworkManager.
pub fn setup(config: &WifiConfig) -> Result<()> {
    // Validate inputs
    validate::connection_name(&config.con_name)?;
    validate::interface_name(&config.iface)?;
    validate::ssid(&config.ssid)?;

    if let Some(ref password) = config.password {
        if config.security == SecurityType::WpaPsk || config.security == SecurityType::Wpa3Sae {
            validate::wpa_password(password)?;
        }
    }

    if let Some(ref ipv4) = config.ipv4 {
        validate::ipv4_address(&ipv4.address)?;
        validate::cidr_prefix(ipv4.prefix)?;
        validate::ipv4_address(&ipv4.gateway)?;
        for dns in &ipv4.dns {
            validate::ipv4_address(dns)?;
        }
    }

    let hidden = if config.is_hidden { "yes" } else { "no" };
    let auto = if config.auto_connect { "yes" } else { "no" };

    info!("Creating WiFi connection '{}'", config.con_name);

    // Step 1: Create the WiFi connection
    nmcli::run(&[
        "c",
        "add",
        "type",
        "wifi",
        "con-name",
        &config.con_name,
        "ifname",
        &config.iface,
        "ssid",
        &config.ssid,
    ])?;

    // Step 2: Configure security based on type
    let mut modify_args: Vec<String> = vec![
        "con".into(),
        "modify".into(),
        config.con_name.clone(),
        "wifi.hidden".into(),
        hidden.into(),
        "connection.autoconnect".into(),
        auto.into(),
    ];

    match config.security {
        SecurityType::Open => {
            // No security configuration needed
        }
        SecurityType::WpaPsk => {
            let password = config.password.as_deref().unwrap_or("");
            modify_args.extend([
                "wifi-sec.key-mgmt".into(),
                "wpa-psk".into(),
                "wifi-sec.psk".into(),
                password.into(),
            ]);
        }
        SecurityType::Wpa3Sae => {
            let password = config.password.as_deref().unwrap_or("");
            modify_args.extend([
                "wifi-sec.key-mgmt".into(),
                "sae".into(),
                "wifi-sec.psk".into(),
                password.into(),
            ]);
        }
        SecurityType::WpaEnterprise => {
            modify_args.extend([
                "wifi-sec.key-mgmt".into(),
                "wpa-eap".into(),
                "802-1x.eap".into(),
                "peap".into(),
                "802-1x.phase2-auth".into(),
                "mschapv2".into(),
            ]);
            if let Some(ref password) = config.password {
                modify_args.extend(["802-1x.password".into(), password.clone()]);
            }
        }
    }

    // Step 3: Configure IP settings
    if let Some(ref ipv4) = config.ipv4 {
        let addr = format!("{}/{}", ipv4.address, ipv4.prefix);
        modify_args.extend([
            "ipv4.method".into(),
            "manual".into(),
            "ipv4.addresses".into(),
            addr,
            "ipv4.gateway".into(),
            ipv4.gateway.clone(),
        ]);
        if !ipv4.dns.is_empty() {
            modify_args.extend([
                "ipv4.dns".into(),
                ipv4.dns.join(" "),
            ]);
        }
    }

    // Step 4: Set connection priority
    if let Some(priority) = config.priority {
        modify_args.extend([
            "connection.autoconnect-priority".into(),
            priority.to_string(),
        ]);
    }

    nmcli::run_owned(&modify_args)?;

    // Step 5: Activate the connection
    info!("Activating connection '{}'", config.con_name);
    nmcli::run(&["con", "up", &config.con_name])?;

    info!("WiFi connection '{}' created and activated", config.con_name);
    Ok(())
}

/// Legacy setup function for backward compatibility.
pub fn setup_simple(
    con_name: String,
    iface: String,
    ssid: String,
    psk: String,
    is_hidden: bool,
    auto_con: bool,
) -> Result<()> {
    let config = WifiConfig {
        con_name,
        iface,
        ssid,
        password: Some(psk),
        security: SecurityType::WpaPsk,
        is_hidden,
        auto_connect: auto_con,
        ipv4: None,
        priority: None,
    };
    setup(&config)
}

/// Removes a network connection by name.
pub fn remove(con_name: &str) -> Result<()> {
    validate::connection_name(con_name)?;
    info!("Removing connection '{con_name}'");
    nmcli::run(&["con", "delete", "id", con_name])?;
    info!("Connection '{con_name}' removed");
    Ok(())
}

/// Scans and lists available WiFi networks.
pub fn scan(iface: Option<&str>) -> Result<Vec<WifiNetwork>> {
    // Trigger a rescan first
    if let Some(iface) = iface {
        validate::interface_name(iface)?;
        let _ = nmcli::run(&["dev", "wifi", "rescan", "ifname", iface]);
    } else {
        let _ = nmcli::run(&["dev", "wifi", "rescan"]);
    }

    // Get list with terse, fields-specified output
    let args = if let Some(iface) = iface {
        vec![
            "-t",
            "-f",
            "IN-USE,SSID,BSSID,SIGNAL,FREQ,SECURITY",
            "dev",
            "wifi",
            "list",
            "ifname",
            iface,
        ]
    } else {
        vec![
            "-t",
            "-f",
            "IN-USE,SSID,BSSID,SIGNAL,FREQ,SECURITY",
            "dev",
            "wifi",
            "list",
        ]
    };

    let output = nmcli::run(&args)?;
    let mut networks = Vec::new();

    for line in output.lines() {
        if line.trim().is_empty() {
            continue;
        }
        // nmcli -t uses ':' as delimiter, but BSSID contains '\:'
        // We need to handle escaped colons in BSSID
        let parts = parse_terse_line(line);
        if parts.len() >= 6 {
            networks.push(WifiNetwork {
                in_use: parts[0].trim() == "*",
                ssid: parts[1].clone(),
                bssid: parts[2].clone(),
                signal: parts[3].parse().unwrap_or(0),
                frequency: parts[4].clone(),
                security: parts[5].clone(),
            });
        }
    }

    Ok(networks)
}

/// Parse a terse nmcli output line, handling escaped colons in BSSID.
fn parse_terse_line(line: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '\\' && i + 1 < chars.len() && chars[i + 1] == ':' {
            current.push(':');
            i += 2;
        } else if chars[i] == ':' {
            parts.push(current.clone());
            current.clear();
            i += 1;
        } else {
            current.push(chars[i]);
            i += 1;
        }
    }
    parts.push(current);
    parts
}

/// Edits an existing WiFi connection's properties.
pub fn edit(con_name: &str, modifications: &[(&str, &str)]) -> Result<()> {
    validate::connection_name(con_name)?;
    if modifications.is_empty() {
        return Err(NetworkError::ValidationError(
            "No modifications specified".into(),
        ));
    }

    let mut args: Vec<String> = vec!["con".into(), "modify".into(), con_name.into()];
    for (key, value) in modifications {
        args.push(key.to_string());
        args.push(value.to_string());
    }

    info!("Modifying connection '{con_name}'");
    nmcli::run_owned(&args)?;
    Ok(())
}

/// Sets the autoconnect priority for a connection.
pub fn set_priority(con_name: &str, priority: i32) -> Result<()> {
    edit(con_name, &[("connection.autoconnect-priority", &priority.to_string())])
}

/// Creates a WiFi hotspot.
pub fn create_hotspot(config: &HotspotConfig) -> Result<String> {
    validate::interface_name(&config.iface)?;
    validate::ssid(&config.ssid)?;

    let mut args = vec![
        "dev",
        "wifi",
        "hotspot",
        "ifname",
        &config.iface,
        "ssid",
        &config.ssid,
    ];

    let password;
    if let Some(ref pw) = config.password {
        password = pw.clone();
        args.extend(["password", &password]);
    }

    let band;
    if let Some(ref b) = config.band {
        band = b.clone();
        args.extend(["band", &band]);
    }

    let channel_str;
    if let Some(ch) = config.channel {
        channel_str = ch.to_string();
        args.extend(["channel", &channel_str]);
    }

    info!("Creating hotspot '{}' on {}", config.ssid, config.iface);
    let output = nmcli::run(&args)?;
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_terse_line() {
        let line = r"*:MyNetwork:AA\:BB\:CC\:DD\:EE\:FF:85:2437 MHz:WPA2";
        let parts = parse_terse_line(line);
        assert_eq!(parts.len(), 6);
        assert_eq!(parts[0], "*");
        assert_eq!(parts[1], "MyNetwork");
        assert_eq!(parts[2], "AA:BB:CC:DD:EE:FF");
        assert_eq!(parts[3], "85");
        assert_eq!(parts[4], "2437 MHz");
        assert_eq!(parts[5], "WPA2");
    }

    #[test]
    fn test_parse_terse_line_empty_ssid() {
        let line = r": :AA\:BB\:CC\:DD\:EE\:FF:50:5180 MHz:WPA2";
        let parts = parse_terse_line(line);
        assert_eq!(parts.len(), 6);
        assert_eq!(parts[0], "");
        assert_eq!(parts[1], " ");
    }
}
