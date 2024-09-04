use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Subcommand to add a new WiFi connection.
    Add {
        #[arg(short, long, required = true, help = "Connection display name.")]
        con_name: String,
        #[arg(short, long, required = true, help = "Wireless interface name.")]
        iface: String,
        #[arg(short, long, required = true, help = "SSID")]
        ssid: String,
        #[arg(long, help = "This a hidden connection.")]
        hidden: bool,
        #[arg(short, long, help = "Automatically connect to the new WiFi connection.")]
        auto_con: bool,
    },
    /// Subcommand to remove a network connection.
    Remove {
        #[arg(short, long, required = true, help = "Connection display name.")]
        con_name: String,
    },
}
