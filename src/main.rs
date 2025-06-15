mod cli;
mod commands;
mod models;
mod storage;

use clap::Parser;
use cli::{Cli, Commands};
use commands::save;

use crate::commands::{list, show};

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Save { name } => {
            save::save_command(name);
        }
        Commands::Run { name } => {
            println!("Would run command '{}'", name);
        }
        Commands::List => {
            list::list_command();
        }
        Commands::Show { name } => {
            show::show_command(name);
        }
    }
}
