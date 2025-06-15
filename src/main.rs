mod cli;
mod commands;
mod models;

use clap::Parser;
use cli::{Cli, Commands};
use commands::save;

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
            println!("Would list all bookmarks");
        }
        Commands::Show { name } => {
            println!("Would show snippet '{}'", name);
        }
    }
}
