mod cli;
mod commands;
mod models;
mod storage;
mod ui;

use clap::Parser;
use cli::{Cli, Commands};

use crate::{
    commands::{copy, delete, edit, export, import, list, restore, run, save, show},
    storage::file_storage::FileStorage,
};

fn main() {
    let args = Cli::parse();
    let storage = FileStorage::new();

    match args.command {
        Commands::Save { name } => {
            save::save_command(&storage, name);
        }
        Commands::Run { name } => {
            run::run_command(&storage, name);
        }
        Commands::List { tag } => {
            list::list_command(&storage, tag);
        }
        Commands::Show { name } => {
            show::show_command(&storage, name);
        }
        Commands::Copy { name } => {
            copy::copy_command(&storage, name);
        }
        Commands::Delete { name, force } => {
            delete::delete_command(&storage, name, force);
        }
        Commands::Edit { name } => {
            edit::edit_command(&storage, name);
        }
        Commands::Export { path } => {
            export::export_command(&storage, &path);
        }
        Commands::Import { path } => {
            import::import_command(&storage, &path);
        }
        Commands::Restore => {
            restore::restore_command(&storage);
        }
    }
}
