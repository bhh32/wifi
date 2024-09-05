pub mod state;
pub mod messages;

use crate::state::AppState;
use std::process::Command;
use rpassword::prompt_password;

/// Sets up the new WiFi network connection using Network Manager
pub fn setup(app_state: &AppState) {
    
    // Get the wifi password (pre-shared key)
    let mut psk = String::new();  
    if app_state.is_cli {
        psk = prompt_password("Enter the wifi password: ").unwrap();
    } else {
        psk = app_state.psk.clone();
    }
    
    // Setup the hidden connection status for nmcli
    let hidden = if app_state.is_hidden { "yes" } else { "no" };

    // Setup the auto connect status for nmcli
    let auto = if app_state.auto_con { "yes" } else { "no" };
    
    // Create the new WiFi network
    Command::new("nmcli")
            .args(["c", "add", "type", "wifi", "con-name", &app_state.con_name, "ifname", &app_state.iface, "ssid", &app_state.ssid])
            .status()
            .expect("Failed to create new connection.");
    
    // Add the security to the new network
    Command::new("nmcli")
            .args(["con", "modify", &app_state.con_name, "wifi-sec.key-mgmt", "wpa-psk", "wifi-sec.psk", &psk, "wifi.hidden", hidden, "connection.autoconnect", auto])
            .status()
            .expect("Failed to set security and autoconnect.");

    // Connect to the new network
    Command::new("nmcli")
            .args(["con", "up", &app_state.con_name])
            .status()
            .expect("Failed to start the new connection: {con_name}");
}

/// Removes a network connection using its name and Network Manager
pub fn remove(app_state: &AppState) {
    // Run the delete command for nmcli
    Command::new("nmcli")
        .args(["con", "delete", "id", &app_state.con_name])
        .status()
        .expect("Failed to remove {con_name}");
}