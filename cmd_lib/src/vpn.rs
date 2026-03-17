use crate::error::{NetworkError, Result};
use crate::nmcli;
use crate::types::{VpnConfig, VpnType};
use crate::validate;
use tracing::info;

/// Sets up a new VPN connection using NetworkManager.
pub fn setup(config: &VpnConfig) -> Result<()> {
    validate::connection_name(&config.con_name)?;

    let auto = if config.auto_connect { "yes" } else { "no" };

    info!("Creating VPN connection '{}'", config.con_name);

    match config.vpn_type {
        VpnType::OpenVpn => setup_openvpn(config, auto),
        VpnType::WireGuard => setup_wireguard(config, auto),
        VpnType::L2tp => setup_l2tp(config, auto),
        VpnType::Pptp => setup_pptp(config, auto),
    }
}

fn setup_openvpn(config: &VpnConfig, auto: &str) -> Result<()> {
    // OpenVPN typically imports from a config file
    if let Some(ref config_file) = config.config_file {
        nmcli::run(&["connection", "import", "type", "openvpn", "file", config_file])?;

        // Rename to desired connection name
        let output = nmcli::run(&["-t", "-f", "NAME", "con", "show", "--active"])?;
        if let Some(imported_name) = output.lines().last() {
            if imported_name.trim() != config.con_name {
                nmcli::run(&[
                    "con",
                    "modify",
                    imported_name.trim(),
                    "connection.id",
                    &config.con_name,
                    "connection.autoconnect",
                    auto,
                ])?;
            }
        }
    } else {
        // Create manually
        nmcli::run(&[
            "c",
            "add",
            "type",
            "vpn",
            "con-name",
            &config.con_name,
            "vpn-type",
            "openvpn",
            "connection.autoconnect",
            auto,
        ])?;

        let mut modify_args: Vec<String> = vec![
            "con".into(),
            "modify".into(),
            config.con_name.clone(),
            "vpn.data".into(),
            format!("remote={}", config.gateway),
        ];

        if let Some(ref username) = config.username {
            modify_args[4] = format!("remote={},username={username}", config.gateway);
        }

        nmcli::run_owned(&modify_args)?;
    }

    if let Some(ref password) = config.password {
        nmcli::run(&[
            "con",
            "modify",
            &config.con_name,
            "vpn.secrets",
            &format!("password={password}"),
        ])?;
    }

    info!("VPN connection '{}' created", config.con_name);
    Ok(())
}

fn setup_wireguard(config: &VpnConfig, auto: &str) -> Result<()> {
    if let Some(ref config_file) = config.config_file {
        nmcli::run(&[
            "connection",
            "import",
            "type",
            "wireguard",
            "file",
            config_file,
        ])?;
        // Set autoconnect
        nmcli::run(&[
            "con",
            "modify",
            &config.con_name,
            "connection.autoconnect",
            auto,
        ])?;
    } else {
        return Err(NetworkError::ValidationError(
            "WireGuard connections require a configuration file".into(),
        ));
    }
    info!("WireGuard connection '{}' created", config.con_name);
    Ok(())
}

fn setup_l2tp(config: &VpnConfig, auto: &str) -> Result<()> {
    nmcli::run(&[
        "c",
        "add",
        "type",
        "vpn",
        "con-name",
        &config.con_name,
        "vpn-type",
        "l2tp",
        "connection.autoconnect",
        auto,
    ])?;

    let mut vpn_data = format!("gateway={}", config.gateway);
    if let Some(ref username) = config.username {
        vpn_data.push_str(&format!(",user={username}"));
    }

    nmcli::run(&[
        "con",
        "modify",
        &config.con_name,
        "vpn.data",
        &vpn_data,
    ])?;

    if let Some(ref password) = config.password {
        nmcli::run(&[
            "con",
            "modify",
            &config.con_name,
            "vpn.secrets",
            &format!("password={password}"),
        ])?;
    }

    info!("L2TP connection '{}' created", config.con_name);
    Ok(())
}

fn setup_pptp(config: &VpnConfig, auto: &str) -> Result<()> {
    nmcli::run(&[
        "c",
        "add",
        "type",
        "vpn",
        "con-name",
        &config.con_name,
        "vpn-type",
        "pptp",
        "connection.autoconnect",
        auto,
    ])?;

    let mut vpn_data = format!("gateway={}", config.gateway);
    if let Some(ref username) = config.username {
        vpn_data.push_str(&format!(",user={username}"));
    }

    nmcli::run(&[
        "con",
        "modify",
        &config.con_name,
        "vpn.data",
        &vpn_data,
    ])?;

    if let Some(ref password) = config.password {
        nmcli::run(&[
            "con",
            "modify",
            &config.con_name,
            "vpn.secrets",
            &format!("password={password}"),
        ])?;
    }

    info!("PPTP connection '{}' created", config.con_name);
    Ok(())
}
