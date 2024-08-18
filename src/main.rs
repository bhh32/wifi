mod cli;
mod logic;

use clap::Parser;
use cli::{Cli, Commands};
use logic::{remove, setup};

fn main() {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Add { con_name, iface, ssid, hidden, auto_con } => setup(con_name.clone(), iface.clone(), ssid.clone(), *hidden, *auto_con),
        Commands::Remove { con_name } => remove(con_name.clone()),
    }
}
