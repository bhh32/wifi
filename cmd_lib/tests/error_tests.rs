use cmd_lib::error::NetworkError;

#[test]
fn test_error_display_command_failed() {
    let err = NetworkError::CommandFailed {
        operation: "con up test".to_string(),
        stderr: "Connection 'test' not found".to_string(),
    };
    let msg = err.to_string();
    assert!(msg.contains("con up test"));
    assert!(msg.contains("not found"));
}

#[test]
fn test_error_display_validation() {
    let err = NetworkError::ValidationError("invalid input".to_string());
    assert!(err.to_string().contains("invalid input"));
}

#[test]
fn test_error_display_connection_not_found() {
    let err = NetworkError::ConnectionNotFound("test-wifi".to_string());
    assert!(err.to_string().contains("test-wifi"));
}

#[test]
fn test_error_display_interface_not_found() {
    let err = NetworkError::InterfaceNotFound("wlan99".to_string());
    assert!(err.to_string().contains("wlan99"));
}

#[test]
fn test_error_display_nm_not_running() {
    let err = NetworkError::NetworkManagerNotRunning;
    assert!(err.to_string().contains("NetworkManager"));
}

#[test]
fn test_error_display_parse_error() {
    let err = NetworkError::ParseError("bad format".to_string());
    assert!(err.to_string().contains("bad format"));
}

#[test]
fn test_error_from_io_error() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "not found");
    let net_err: NetworkError = io_err.into();
    assert!(matches!(net_err, NetworkError::CommandExecution(_)));
    assert!(net_err.to_string().contains("nmcli"));
}

#[test]
fn test_error_source() {
    use std::error::Error;

    let io_err = std::io::Error::new(std::io::ErrorKind::Other, "test");
    let net_err = NetworkError::CommandExecution(io_err);
    assert!(net_err.source().is_some());

    let validation_err = NetworkError::ValidationError("test".to_string());
    assert!(validation_err.source().is_none());
}
