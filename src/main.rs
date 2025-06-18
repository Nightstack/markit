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
    ui::{cli_selection::CliSelection, cli_table::CliTable},
};

fn main() {
    let args = Cli::parse();
    let storage = FileStorage::new();

    match args.command {
        Commands::Save { name } => {
            save::save_command(&storage, name);
        }
        Commands::Run { name } => {
            let selection_ui = CliSelection::new();
            run::run_command(&storage, &selection_ui, name);
        }
        Commands::List { tag } => {
            let mut cli_table = CliTable::new();
            list::list_command(&storage, &mut cli_table, tag);
        }
        Commands::Show { name } => {
            let selection_ui = CliSelection::new();
            show::show_command(&storage, &selection_ui, name);
        }
        Commands::Copy { name } => {
            let selection_ui = CliSelection::new();
            copy::copy_command(&storage, &selection_ui, name);
        }
        Commands::Delete { name, force } => {
            let selection_ui = CliSelection::new();
            delete::delete_command(&storage, &selection_ui, name, force);
        }
        Commands::Edit { name } => {
            let selection_ui = CliSelection::new();
            edit::edit_command(&storage, &selection_ui, name);
        }
        Commands::Export { path } => {
            export::export_command(&storage, &path);
        }
        Commands::Import { path } => {
            import::import_command(&storage, &path);
        }
        Commands::Restore => {
            let selection_ui = CliSelection::new();
            restore::restore_command(&storage, &selection_ui);
        }
    }
}
