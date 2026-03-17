//! # cmd_lib
//!
//! Core library for managing network connections via Linux NetworkManager (nmcli).
//!
//! Provides modules for WiFi, Ethernet, and VPN connection management,
//! along with network scanning, interface detection, and connection profile
//! import/export.

pub mod connection;
pub mod error;
pub mod ethernet;
pub mod nmcli;
pub mod types;
pub mod validate;
pub mod vpn;
pub mod wifi;

#[cfg(feature = "async")]
pub mod async_nmcli;

// Re-export commonly used items for convenience
pub use error::{NetworkError, Result};
pub use types::*;

// Legacy API for backward compatibility
pub use wifi::setup_simple as setup;

/// Removes a connection by name. Legacy wrapper.
pub fn remove(con_name: String) -> Result<()> {
    connection::remove(&con_name)
}
