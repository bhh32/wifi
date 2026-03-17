use cmd_lib::validate;

#[test]
fn test_connection_name_valid() {
    assert!(validate::connection_name("MyConnection").is_ok());
    assert!(validate::connection_name("my-wifi-5g").is_ok());
    assert!(validate::connection_name("home_network.2").is_ok());
    assert!(validate::connection_name("Office WiFi").is_ok());
    assert!(validate::connection_name("a").is_ok());
}

#[test]
fn test_connection_name_empty() {
    let result = validate::connection_name("");
    assert!(result.is_err());
}

#[test]
fn test_connection_name_too_long() {
    let long_name = "a".repeat(129);
    assert!(validate::connection_name(&long_name).is_err());
    let max_name = "a".repeat(128);
    assert!(validate::connection_name(&max_name).is_ok());
}

#[test]
fn test_connection_name_injection_attempts() {
    assert!(validate::connection_name("net; rm -rf /").is_err());
    assert!(validate::connection_name("net`whoami`").is_err());
    assert!(validate::connection_name("net$(cmd)").is_err());
    assert!(validate::connection_name("net|cat /etc/passwd").is_err());
    assert!(validate::connection_name("net&bg").is_err());
    assert!(validate::connection_name("net\nnewline").is_err());
}

#[test]
fn test_interface_name_valid() {
    assert!(validate::interface_name("wlan0").is_ok());
    assert!(validate::interface_name("eth0").is_ok());
    assert!(validate::interface_name("wlp2s0").is_ok());
    assert!(validate::interface_name("enp0s3").is_ok());
    assert!(validate::interface_name("br-lan").is_ok());
    assert!(validate::interface_name("tun0").is_ok());
}

#[test]
fn test_interface_name_invalid() {
    assert!(validate::interface_name("").is_err());
    assert!(validate::interface_name("this_is_too_long_name").is_err());
    assert!(validate::interface_name("eth 0").is_err());
    assert!(validate::interface_name("eth;0").is_err());
}

#[test]
fn test_ssid_valid() {
    assert!(validate::ssid("MyNetwork").is_ok());
    assert!(validate::ssid("a").is_ok());
    assert!(validate::ssid(&"a".repeat(32)).is_ok());
    assert!(validate::ssid("Network With Spaces!").is_ok());
}

#[test]
fn test_ssid_invalid() {
    assert!(validate::ssid("").is_err());
    assert!(validate::ssid(&"a".repeat(33)).is_err());
}

#[test]
fn test_wpa_password_valid() {
    assert!(validate::wpa_password("12345678").is_ok());
    assert!(validate::wpa_password("a strong password here").is_ok());
    assert!(validate::wpa_password(&"x".repeat(63)).is_ok());
}

#[test]
fn test_wpa_password_invalid() {
    assert!(validate::wpa_password("short").is_err());
    assert!(validate::wpa_password("1234567").is_err());
    assert!(validate::wpa_password(&"x".repeat(64)).is_err());
}

#[test]
fn test_ipv4_address_valid() {
    assert!(validate::ipv4_address("192.168.1.1").is_ok());
    assert!(validate::ipv4_address("10.0.0.1").is_ok());
    assert!(validate::ipv4_address("255.255.255.0").is_ok());
    assert!(validate::ipv4_address("0.0.0.0").is_ok());
    assert!(validate::ipv4_address("8.8.8.8").is_ok());
}

#[test]
fn test_ipv4_address_invalid() {
    assert!(validate::ipv4_address("256.1.1.1").is_err());
    assert!(validate::ipv4_address("1.2.3").is_err());
    assert!(validate::ipv4_address("1.2.3.4.5").is_err());
    assert!(validate::ipv4_address("not.an.ip.addr").is_err());
    assert!(validate::ipv4_address("").is_err());
    assert!(validate::ipv4_address("192.168.1").is_err());
}

#[test]
fn test_cidr_prefix_valid() {
    assert!(validate::cidr_prefix(0).is_ok());
    assert!(validate::cidr_prefix(8).is_ok());
    assert!(validate::cidr_prefix(16).is_ok());
    assert!(validate::cidr_prefix(24).is_ok());
    assert!(validate::cidr_prefix(32).is_ok());
}

#[test]
fn test_cidr_prefix_invalid() {
    assert!(validate::cidr_prefix(33).is_err());
    assert!(validate::cidr_prefix(64).is_err());
    assert!(validate::cidr_prefix(255).is_err());
}
