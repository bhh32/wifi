mod cli; // Add the CLI module

// Import clap, cli, and logic stuff
use clap::Parser;
use cli::{Cli, Commands};
use cmd_lib::{state::AppState, remove, setup};

fn main() {
    // Get the arguments given to the CLI
    let cli = Cli::parse();
    
    // Match the commands
    match &cli.command {
        // If it's the add command, parse the arguments and then do the logic
        Commands::Add { con_name, iface, ssid, hidden, auto_con } => {
            let mut app_state = AppState {
                con_name: con_name.to_string(),
                iface: iface.to_string(),
                ssid: ssid.to_string(),
                is_hidden: *hidden,
                auto_con: *auto_con,
                ..Default::default()
            };
            setup(&mut app_state);
        },
        // If it's remove, parse the argument and do the logic
        Commands::Remove { con_name } => {
            let app_state = AppState {
                con_name: con_name.to_string(),
                ..Default::default()
            };
            remove(&app_state);
        }
    }
}
