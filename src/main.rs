mod cli;
mod clipboard_provider;
mod command_runner;
mod commands;
mod file;
mod input;
mod models;
mod storage;
mod ui;

use clap::Parser;
use cli::{Cli, Commands};

use crate::{
    clipboard_provider::SmartClipboard,
    command_runner::ShellCommandRunner,
    commands::{copy, delete, edit, export, import, list, restore, run, save, show},
    file::{editor::Editor, reader::Reader, writer::Writer},
    input::cli_save::CliSaveInput,
    storage::file_storage::FileStorage,
    ui::{cli_confirm::DialoguerConfirm, cli_selection::CliSelection, cli_table::CliTable},
};

fn main() {
    let args = Cli::parse();
    let storage = FileStorage::new();

    match args.command {
        Commands::Save { name } => {
            let input = CliSaveInput;
            save::save_command(&storage, &input, name);
        }
        Commands::Run { name } => {
            let selection_ui = CliSelection::new();
            let runner = ShellCommandRunner;
            run::run_command(&storage, &selection_ui, &runner, name);
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
            let mut clipboard = SmartClipboard::new();
            copy::copy_command(&storage, &selection_ui, &mut clipboard, name);
        }
        Commands::Delete { name, force } => {
            let selection_ui = CliSelection::new();
            let confirm_prompt = DialoguerConfirm;
            delete::delete_command(&storage, &selection_ui, &confirm_prompt, name, force);
        }
        Commands::Edit { name } => {
            let selection_ui = CliSelection::new();
            let editor = Editor;
            edit::edit_command(&storage, &selection_ui, &editor, name);
        }
        Commands::Export { path } => {
            let writer = Writer;
            export::export_command(&storage, &writer, &path);
        }
        Commands::Import { path } => {
            let reader = Reader;
            import::import_command(&storage, &reader, &path);
        }
        Commands::Restore => {
            let selection_ui = CliSelection::new();
            restore::restore_command(&storage, &selection_ui);
        }
    }
}
