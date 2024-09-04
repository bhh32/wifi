mod cli; // Add the CLI module

// Import clap, cli, and logic stuff
use clap::Parser;
use cli::{Cli, Commands};
use cmd_lib::{remove, setup};

fn main() {
    // Get the arguments given to the CLI
    let cli = Cli::parse();
    
    // Match the commands
    match &cli.command {
        // If it's the add command, parse the arguments and then do the logic
        Commands::Add { con_name, iface, ssid, hidden, auto_con } => setup(con_name.clone(), iface.clone(), ssid.clone(), *hidden, *auto_con),
        // If it's remove, parse the argument and do the logic
        Commands::Remove { con_name } => remove(con_name.clone()),
    }
}
