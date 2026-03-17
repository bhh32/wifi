use std::fmt;

/// Errors that can occur during network management operations.
#[derive(Debug)]
pub enum NetworkError {
    /// Failed to execute the nmcli command
    CommandExecution(std::io::Error),
    /// nmcli command returned a non-zero exit code
    CommandFailed { operation: String, stderr: String },
    /// Input validation failed
    ValidationError(String),
    /// Failed to parse nmcli output
    ParseError(String),
    /// The requested connection was not found
    ConnectionNotFound(String),
    /// The requested interface was not found
    InterfaceNotFound(String),
    /// NetworkManager is not running
    NetworkManagerNotRunning,
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CommandExecution(e) => write!(f, "Failed to execute nmcli: {e}"),
            Self::CommandFailed { operation, stderr } => {
                write!(f, "nmcli {operation} failed: {stderr}")
            }
            Self::ValidationError(msg) => write!(f, "Validation error: {msg}"),
            Self::ParseError(msg) => write!(f, "Parse error: {msg}"),
            Self::ConnectionNotFound(name) => write!(f, "Connection '{name}' not found"),
            Self::InterfaceNotFound(name) => write!(f, "Interface '{name}' not found"),
            Self::NetworkManagerNotRunning => write!(f, "NetworkManager is not running"),
        }
    }
}

impl std::error::Error for NetworkError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::CommandExecution(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for NetworkError {
    fn from(err: std::io::Error) -> Self {
        Self::CommandExecution(err)
    }
}

pub type Result<T> = std::result::Result<T, NetworkError>;
