use std::process::Command;
use rpassword::prompt_password;

/// Sets up the new WiFi network connection using Network Manager
pub fn setup(con_name: String, iface: String, ssid: String, is_hidden: bool, auto_con: bool) {
    // Get the wifi password (pre-shared key)
    let psk = prompt_password("Enter the wifi password: ").unwrap();
    
    // Setup the hidden connection status for nmcli
    let hidden = if is_hidden { "yes" } else { "no" };

    // Setup the auto connect status for nmcli
    let auto = if auto_con { "yes" } else { "no" };
    
    // Create the new WiFi network
    Command::new("nmcli")
            .args(["c", "add", "type", "wifi", "con-name", &con_name, "ifname", &iface, "ssid", &ssid])
            .status()
            .expect("Failed to create new connection.");
    
    // Add the security to the new network
    Command::new("nmcli")
            .args(["con", "modify", &con_name, "wifi-sec.key-mgmt", "wpa-psk", "wifi-sec.psk", &psk, "wifi.hidden", hidden, "connection.autoconnect", auto])
            .status()
            .expect("Failed to set security and autoconnect.");

    // Connect to the new network
    Command::new("nmcli")
            .args(["con", "up", &con_name])
            .status()
            .expect("Failed to start the new connection: {con_name}");
}

/// Removes a network connection using its name and Network Manager
pub fn remove(con_name: String) {
    // Run the delete command for nmcli
    Command::new("nmcli")
        .args(["con", "delete", "id", &con_name])
        .status()
        .expect("Failed to remove {con_name}");
}