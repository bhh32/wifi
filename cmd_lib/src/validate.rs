use crate::error::{NetworkError, Result};

/// Validates a connection name. Must be non-empty and contain only safe characters.
pub fn connection_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(NetworkError::ValidationError(
            "Connection name cannot be empty".into(),
        ));
    }
    if name.len() > 128 {
        return Err(NetworkError::ValidationError(
            "Connection name must be 128 characters or less".into(),
        ));
    }
    // Allow alphanumeric, spaces, hyphens, underscores, dots
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || " -_.".contains(c))
    {
        return Err(NetworkError::ValidationError(
            "Connection name contains invalid characters. Use alphanumeric, spaces, hyphens, underscores, or dots.".into(),
        ));
    }
    Ok(())
}

/// Validates an interface name.
pub fn interface_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(NetworkError::ValidationError(
            "Interface name cannot be empty".into(),
        ));
    }
    if name.len() > 15 {
        return Err(NetworkError::ValidationError(
            "Interface name must be 15 characters or less".into(),
        ));
    }
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        return Err(NetworkError::ValidationError(
            "Interface name contains invalid characters".into(),
        ));
    }
    Ok(())
}

/// Validates an SSID.
pub fn ssid(ssid: &str) -> Result<()> {
    if ssid.is_empty() {
        return Err(NetworkError::ValidationError(
            "SSID cannot be empty".into(),
        ));
    }
    if ssid.len() > 32 {
        return Err(NetworkError::ValidationError(
            "SSID must be 32 characters or less".into(),
        ));
    }
    Ok(())
}

/// Validates a WPA password.
pub fn wpa_password(password: &str) -> Result<()> {
    if password.len() < 8 {
        return Err(NetworkError::ValidationError(
            "WPA password must be at least 8 characters".into(),
        ));
    }
    if password.len() > 63 {
        return Err(NetworkError::ValidationError(
            "WPA password must be 63 characters or less".into(),
        ));
    }
    Ok(())
}

/// Validates an IPv4 address string.
pub fn ipv4_address(addr: &str) -> Result<()> {
    let parts: Vec<&str> = addr.split('.').collect();
    if parts.len() != 4 {
        return Err(NetworkError::ValidationError(format!(
            "Invalid IPv4 address: {addr}"
        )));
    }
    for part in parts {
        match part.parse::<u8>() {
            Ok(_) => {}
            Err(_) => {
                return Err(NetworkError::ValidationError(format!(
                    "Invalid IPv4 address octet: {part}"
                )));
            }
        }
    }
    Ok(())
}

/// Validates a CIDR prefix length.
pub fn cidr_prefix(prefix: u32) -> Result<()> {
    if prefix > 32 {
        return Err(NetworkError::ValidationError(format!(
            "CIDR prefix must be 0-32, got {prefix}"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_connection_name() {
        assert!(connection_name("My-WiFi_Network.5g").is_ok());
    }

    #[test]
    fn test_empty_connection_name() {
        assert!(connection_name("").is_err());
    }

    #[test]
    fn test_invalid_connection_name_chars() {
        assert!(connection_name("net;rm -rf /").is_err());
        assert!(connection_name("net`whoami`").is_err());
        assert!(connection_name("net$(cmd)").is_err());
    }

    #[test]
    fn test_valid_interface_name() {
        assert!(interface_name("wlan0").is_ok());
        assert!(interface_name("eth0").is_ok());
        assert!(interface_name("wlp2s0").is_ok());
    }

    #[test]
    fn test_invalid_interface_name() {
        assert!(interface_name("").is_err());
        assert!(interface_name("this-name-is-way-too-long").is_err());
    }

    #[test]
    fn test_valid_ssid() {
        assert!(ssid("MyNetwork").is_ok());
        assert!(ssid("a").is_ok());
    }

    #[test]
    fn test_invalid_ssid() {
        assert!(ssid("").is_err());
        assert!(ssid(&"a".repeat(33)).is_err());
    }

    #[test]
    fn test_valid_wpa_password() {
        assert!(wpa_password("password123").is_ok());
    }

    #[test]
    fn test_invalid_wpa_password() {
        assert!(wpa_password("short").is_err());
        assert!(wpa_password(&"a".repeat(64)).is_err());
    }

    #[test]
    fn test_valid_ipv4() {
        assert!(ipv4_address("192.168.1.1").is_ok());
        assert!(ipv4_address("10.0.0.1").is_ok());
    }

    #[test]
    fn test_invalid_ipv4() {
        assert!(ipv4_address("999.1.1.1").is_err());
        assert!(ipv4_address("not.an.ip").is_err());
        assert!(ipv4_address("1.2.3").is_err());
    }

    #[test]
    fn test_valid_cidr() {
        assert!(cidr_prefix(24).is_ok());
        assert!(cidr_prefix(0).is_ok());
        assert!(cidr_prefix(32).is_ok());
    }

    #[test]
    fn test_invalid_cidr() {
        assert!(cidr_prefix(33).is_err());
    }
}
