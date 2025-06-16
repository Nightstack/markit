mod cli;
mod commands;
mod models;
mod storage;
mod ui;

use clap::Parser;
use cli::{Cli, Commands};

use crate::commands::{copy, delete, edit, export, import, list, restore, run, save, show};

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Save { name } => {
            save::save_command(name);
        }
        Commands::Run { name } => {
            run::run_command(name);
        }
        Commands::List { tag } => {
            list::list_command(tag);
        }
        Commands::Show { name } => {
            show::show_command(name);
        }
        Commands::Copy { name } => {
            copy::copy_command(name);
        }
        Commands::Delete { name, force } => {
            delete::delete_command(name, force);
        }
        Commands::Edit { name } => {
            edit::edit_command(name);
        }
        Commands::Export { path } => {
            export::export_command(&path);
        }
        Commands::Import { path } => {
            import::import_command(&path);
        }
        Commands::Restore => {
            restore::restore_command();
        }
    }
}
