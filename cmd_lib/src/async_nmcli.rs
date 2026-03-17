//! Async versions of nmcli operations using tokio.
//!
//! Enable with the `async` feature flag.

#[cfg(feature = "async")]
use crate::error::{NetworkError, Result};
#[cfg(feature = "async")]
use tokio::process::Command;
#[cfg(feature = "async")]
use tracing::{debug, error};

/// Execute an nmcli command asynchronously and return stdout on success.
#[cfg(feature = "async")]
pub async fn run(args: &[&str]) -> Result<String> {
    debug!("Running async: nmcli {}", args.join(" "));

    let output = Command::new("nmcli").args(args).output().await?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if output.status.success() {
        debug!("nmcli stdout: {stdout}");
        Ok(stdout)
    } else {
        let operation = args.join(" ");
        error!("nmcli {operation} failed: {stderr}");
        Err(NetworkError::CommandFailed {
            operation,
            stderr: stderr.trim().to_string(),
        })
    }
}

/// Execute an nmcli command asynchronously with owned String args.
#[cfg(feature = "async")]
pub async fn run_owned(args: &[String]) -> Result<String> {
    let refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    run(&refs).await
}

/// Check if NetworkManager is running (async version).
#[cfg(feature = "async")]
pub async fn check_nm_running() -> Result<()> {
    let output = Command::new("nmcli")
        .args(["general", "status"])
        .output()
        .await?;

    if !output.status.success() {
        return Err(NetworkError::NetworkManagerNotRunning);
    }
    Ok(())
}
