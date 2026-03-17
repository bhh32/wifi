use crate::error::Result;
use crate::nmcli;
use crate::types::EthernetConfig;
use crate::validate;
use tracing::info;

/// Sets up a new Ethernet connection using NetworkManager.
pub fn setup(config: &EthernetConfig) -> Result<()> {
    validate::connection_name(&config.con_name)?;
    validate::interface_name(&config.iface)?;

    if let Some(ref ipv4) = config.ipv4 {
        validate::ipv4_address(&ipv4.address)?;
        validate::cidr_prefix(ipv4.prefix)?;
        validate::ipv4_address(&ipv4.gateway)?;
        for dns in &ipv4.dns {
            validate::ipv4_address(dns)?;
        }
    }

    let auto = if config.auto_connect { "yes" } else { "no" };

    info!("Creating Ethernet connection '{}'", config.con_name);

    // Create the Ethernet connection
    nmcli::run(&[
        "c",
        "add",
        "type",
        "ethernet",
        "con-name",
        &config.con_name,
        "ifname",
        &config.iface,
        "connection.autoconnect",
        auto,
    ])?;

    // Configure IP settings if manual
    if let Some(ref ipv4) = config.ipv4 {
        let addr = format!("{}/{}", ipv4.address, ipv4.prefix);
        let mut args: Vec<String> = vec![
            "con".into(),
            "modify".into(),
            config.con_name.clone(),
            "ipv4.method".into(),
            "manual".into(),
            "ipv4.addresses".into(),
            addr,
            "ipv4.gateway".into(),
            ipv4.gateway.clone(),
        ];
        if !ipv4.dns.is_empty() {
            args.extend(["ipv4.dns".into(), ipv4.dns.join(" ")]);
        }
        nmcli::run_owned(&args)?;
    }

    // Set priority if specified
    if let Some(priority) = config.priority {
        nmcli::run(&[
            "con",
            "modify",
            &config.con_name,
            "connection.autoconnect-priority",
            &priority.to_string(),
        ])?;
    }

    // Activate the connection
    info!("Activating Ethernet connection '{}'", config.con_name);
    nmcli::run(&["con", "up", &config.con_name])?;

    info!(
        "Ethernet connection '{}' created and activated",
        config.con_name
    );
    Ok(())
}
