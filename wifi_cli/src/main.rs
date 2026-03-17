mod cli;

use clap::Parser;
use cli::{Cli, Commands};
use cmd_lib::types::{
    ConnectionProfile, EthernetConfig, HotspotConfig, Ipv4Config, VpnConfig, WifiConfig,
};
use cmd_lib::{connection, ethernet, vpn, wifi};
use rpassword::prompt_password;
use std::process;

fn main() {
    let cli = Cli::parse();

    // Initialize logging
    if cli.verbose {
        tracing_subscriber::fmt()
            .with_env_filter("cmd_lib=debug")
            .init();
    }

    if let Err(e) = run_command(cli.command) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn run_command(command: Commands) -> cmd_lib::Result<()> {
    match command {
        Commands::Add {
            con_name,
            iface,
            ssid,
            hidden,
            auto_con,
            security,
            ipv4,
            gateway,
            dns,
            priority,
        } => {
            let iface = resolve_wifi_interface(iface)?;
            let security_type: cmd_lib::SecurityType = security.into();

            let password = if security_type != cmd_lib::SecurityType::Open {
                let pw = prompt_password("Enter the Wi-Fi password: ").map_err(|e| {
                    cmd_lib::NetworkError::ValidationError(format!("Failed to read password: {e}"))
                })?;
                if pw.is_empty() {
                    return Err(cmd_lib::NetworkError::ValidationError(
                        "Password cannot be empty for secured networks".into(),
                    ));
                }
                Some(pw)
            } else {
                None
            };

            let ipv4_config = parse_ipv4_config(ipv4, gateway, dns)?;

            let config = WifiConfig {
                con_name,
                iface,
                ssid,
                password,
                security: security_type,
                is_hidden: hidden,
                auto_connect: auto_con,
                ipv4: ipv4_config,
                priority,
            };

            wifi::setup(&config)?;
            println!("WiFi connection '{}' created successfully.", config.con_name);
        }

        Commands::AddEthernet {
            con_name,
            iface,
            auto_con,
            ipv4,
            gateway,
            dns,
            priority,
        } => {
            let iface = resolve_ethernet_interface(iface)?;
            let ipv4_config = parse_ipv4_config(ipv4, gateway, dns)?;

            let config = EthernetConfig {
                con_name: con_name.clone(),
                iface,
                auto_connect: auto_con,
                ipv4: ipv4_config,
                priority,
            };

            ethernet::setup(&config)?;
            println!("Ethernet connection '{con_name}' created successfully.");
        }

        Commands::AddVpn {
            con_name,
            vpn_type,
            gateway,
            username,
            config_file,
            auto_con,
        } => {
            let password = if config_file.is_none() {
                let pw = prompt_password("Enter the VPN password: ").map_err(|e| {
                    cmd_lib::NetworkError::ValidationError(format!("Failed to read password: {e}"))
                })?;
                Some(pw)
            } else {
                None
            };

            let config = VpnConfig {
                con_name: con_name.clone(),
                vpn_type: vpn_type.into(),
                gateway,
                username,
                password,
                config_file,
                auto_connect: auto_con,
            };

            vpn::setup(&config)?;
            println!("VPN connection '{con_name}' created successfully.");
        }

        Commands::Remove { con_name } => {
            connection::remove(&con_name)?;
            println!("Connection '{con_name}' removed.");
        }

        Commands::Edit {
            con_name,
            auto_con,
            priority,
            ipv4,
            gateway,
            dns,
        } => {
            let mut modifications: Vec<(&str, String)> = Vec::new();

            if let Some(auto) = auto_con {
                modifications.push((
                    "connection.autoconnect",
                    if auto { "yes" } else { "no" }.into(),
                ));
            }
            if let Some(p) = priority {
                modifications.push(("connection.autoconnect-priority", p.to_string()));
            }
            if let Some(ref addr) = ipv4 {
                modifications.push(("ipv4.method", "manual".into()));
                modifications.push(("ipv4.addresses", addr.clone()));
            }
            if let Some(ref gw) = gateway {
                modifications.push(("ipv4.gateway", gw.clone()));
            }
            if let Some(ref d) = dns {
                modifications.push(("ipv4.dns", d.replace(',', " ")));
            }

            if modifications.is_empty() {
                eprintln!("No modifications specified. Use --help for options.");
                process::exit(1);
            }

            let mod_refs: Vec<(&str, &str)> =
                modifications.iter().map(|(k, v)| (*k, v.as_str())).collect();
            wifi::edit(&con_name, &mod_refs)?;
            println!("Connection '{con_name}' modified.");
        }

        Commands::Scan { iface } => {
            let networks = wifi::scan(iface.as_deref())?;
            if networks.is_empty() {
                println!("No WiFi networks found.");
            } else {
                println!(
                    "{:<4} {:<32} {:<19} {:<8} {:<12} {}",
                    "", "SSID", "BSSID", "SIGNAL", "FREQ", "SECURITY"
                );
                println!("{}", "-".repeat(85));
                for net in &networks {
                    let in_use = if net.in_use { " *" } else { "  " };
                    println!(
                        "{:<4} {:<32} {:<19} {:<8} {:<12} {}",
                        in_use, net.ssid, net.bssid, net.signal, net.frequency, net.security
                    );
                }
                println!("\n{} network(s) found.", networks.len());
            }
        }

        Commands::List => {
            let connections = connection::list()?;
            if connections.is_empty() {
                println!("No saved connections.");
            } else {
                println!(
                    "{:<30} {:<38} {:<15} {:<12} {}",
                    "NAME", "UUID", "TYPE", "DEVICE", "ACTIVE"
                );
                println!("{}", "-".repeat(100));
                for con in &connections {
                    let active = if con.active { "yes" } else { "no" };
                    let device = if con.device.is_empty() {
                        "--"
                    } else {
                        &con.device
                    };
                    println!(
                        "{:<30} {:<38} {:<15} {:<12} {}",
                        con.name, con.uuid, con.conn_type, device, active
                    );
                }
            }
        }

        Commands::Status { device } => {
            let statuses = connection::status(device.as_deref())?;
            if statuses.is_empty() {
                println!("No devices found.");
            } else {
                for s in &statuses {
                    println!("Device:     {}", s.device);
                    println!("Type:       {}", s.conn_type);
                    println!("State:      {}", s.state);
                    println!("Connection: {}", s.connection);
                    if let Some(ref ssid) = s.ssid {
                        println!("SSID:       {ssid}");
                    }
                    if let Some(sig) = s.signal {
                        println!("Signal:     {sig}%");
                    }
                    if let Some(ref addr) = s.ip4_address {
                        println!("IPv4:       {addr}");
                    }
                    if let Some(ref gw) = s.ip4_gateway {
                        println!("Gateway:    {gw}");
                    }
                    if !s.ip4_dns.is_empty() {
                        println!("DNS:        {}", s.ip4_dns.join(", "));
                    }
                    println!();
                }
            }
        }

        Commands::Interfaces => {
            let interfaces = connection::list_interfaces()?;
            if interfaces.is_empty() {
                println!("No interfaces found.");
            } else {
                println!("{:<15} {:<12} {:<15} {}", "DEVICE", "TYPE", "STATE", "CONNECTION");
                println!("{}", "-".repeat(60));
                for iface in &interfaces {
                    let con = iface.connection.as_deref().unwrap_or("--");
                    println!(
                        "{:<15} {:<12} {:<15} {}",
                        iface.name, iface.iface_type, iface.state, con
                    );
                }
            }
        }

        Commands::Hotspot {
            iface,
            ssid,
            password,
            band,
            channel,
        } => {
            let config = HotspotConfig {
                iface,
                ssid: ssid.clone(),
                password,
                band,
                channel,
            };
            wifi::create_hotspot(&config)?;
            println!("Hotspot '{ssid}' created.");
        }

        Commands::Up { con_name } => {
            connection::activate(&con_name)?;
            println!("Connection '{con_name}' activated.");
        }

        Commands::Down { con_name } => {
            connection::deactivate(&con_name)?;
            println!("Connection '{con_name}' deactivated.");
        }

        Commands::Export { con_name, output } => {
            let profile = connection::export_profile(&con_name)?;
            let json = serde_json::to_string_pretty(&profile).map_err(|e| {
                cmd_lib::NetworkError::ParseError(format!("Failed to serialize profile: {e}"))
            })?;

            if let Some(path) = output {
                std::fs::write(&path, &json).map_err(|e| {
                    cmd_lib::NetworkError::CommandExecution(e)
                })?;
                println!("Profile exported to '{path}'.");
            } else {
                println!("{json}");
            }
        }

        Commands::Import { file } => {
            let json = std::fs::read_to_string(&file).map_err(|e| {
                cmd_lib::NetworkError::CommandExecution(e)
            })?;
            let profile: ConnectionProfile = serde_json::from_str(&json).map_err(|e| {
                cmd_lib::NetworkError::ParseError(format!("Failed to parse profile: {e}"))
            })?;
            connection::import_profile(&profile)?;
            println!("Profile '{}' imported.", profile.name);
        }
    }

    Ok(())
}

/// Parse IPv4 config from CLI arguments.
fn parse_ipv4_config(
    ipv4: Option<String>,
    gateway: Option<String>,
    dns: Option<String>,
) -> cmd_lib::Result<Option<Ipv4Config>> {
    if let Some(addr_str) = ipv4 {
        let (address, prefix) = if let Some((a, p)) = addr_str.split_once('/') {
            (
                a.to_string(),
                p.parse::<u32>().map_err(|_| {
                    cmd_lib::NetworkError::ValidationError(format!(
                        "Invalid CIDR prefix: {p}"
                    ))
                })?,
            )
        } else {
            (addr_str, 24)
        };

        let gateway = gateway.unwrap_or_default();
        let dns_list: Vec<String> = dns
            .map(|d| d.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default();

        Ok(Some(Ipv4Config {
            address,
            prefix,
            gateway,
            dns: dns_list,
        }))
    } else {
        Ok(None)
    }
}

/// Resolve WiFi interface — use provided or auto-detect.
fn resolve_wifi_interface(iface: Option<String>) -> cmd_lib::Result<String> {
    if let Some(iface) = iface {
        return Ok(iface);
    }
    let wifi_interfaces = connection::detect_wifi_interfaces()?;
    if let Some(first) = wifi_interfaces.first() {
        println!("Auto-detected WiFi interface: {}", first.name);
        Ok(first.name.clone())
    } else {
        Err(cmd_lib::NetworkError::InterfaceNotFound(
            "No WiFi interfaces detected. Specify one with --iface.".into(),
        ))
    }
}

/// Resolve Ethernet interface — use provided or auto-detect.
fn resolve_ethernet_interface(iface: Option<String>) -> cmd_lib::Result<String> {
    if let Some(iface) = iface {
        return Ok(iface);
    }
    let eth_interfaces = connection::detect_ethernet_interfaces()?;
    if let Some(first) = eth_interfaces.first() {
        println!("Auto-detected Ethernet interface: {}", first.name);
        Ok(first.name.clone())
    } else {
        Err(cmd_lib::NetworkError::InterfaceNotFound(
            "No Ethernet interfaces detected. Specify one with --iface.".into(),
        ))
    }
}
