mod cli;
mod commands;
mod models;
mod storage;
mod ui;

use clap::Parser;
use cli::{Cli, Commands};

use crate::commands::{copy, delete, list, run, save, show};

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Save { name } => {
            save::save_command(name);
        }
        Commands::Run { name } => {
            run::run_command(name);
        }
        Commands::List => {
            list::list_command();
        }
        Commands::Show { name } => {
            show::show_command(name);
        }
        Commands::Copy { name } => {
            copy::copy_command(name);
        }
        Commands::Delete { name } => {
            delete::delete_command(name);
        }
    }
}
