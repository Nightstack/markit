use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "markit")]
#[command(about = "A CLI snippet runner/bookmarker", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Save a new snippet interactively")]
    Save { name: String },

    #[command(about = "List all saved snippets (optionally filter by tag)")]
    List {
        #[arg(short, long, help = "Filter by tag")]
        tag: Option<String>,
    },

    #[command(about = "Show the full content of a snippet")]
    Show { name: String },

    #[command(about = "Run a saved snippet")]
    Run { name: String },

    #[command(about = "Edit a saved snippet in your default editor")]
    Edit { name: String },

    #[command(about = "Delete a snippet with confirmation prompt")]
    Delete {
        name: String,

        #[arg(short, long, help = "Force delete without confirmation")]
        force: bool,
    },

    #[command(about = "Copy a snippet's content to the clipboard")]
    Copy { name: String },

    #[command(about = "Export all snippets to a YAML file")]
    Export { path: String },

    #[command(about = "Import snippets from a YAML file")]
    Import { path: String },

    #[command(about = "Restore a previous backup")]
    Restore,
}
