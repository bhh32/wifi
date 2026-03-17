use crate::error::{NetworkError, Result};
use std::process::Command;
use tracing::{debug, error};

/// Execute an nmcli command and return stdout on success.
pub fn run(args: &[&str]) -> Result<String> {
    debug!("Running: nmcli {}", args.join(" "));

    let output = Command::new("nmcli").args(args).output()?;

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

/// Execute an nmcli command with owned String args.
pub fn run_owned(args: &[String]) -> Result<String> {
    let refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    run(&refs)
}

/// Check if NetworkManager is running.
pub fn check_nm_running() -> Result<()> {
    let output = Command::new("nmcli").args(["general", "status"]).output()?;

    if !output.status.success() {
        return Err(NetworkError::NetworkManagerNotRunning);
    }
    Ok(())
}
