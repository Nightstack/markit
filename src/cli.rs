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
    Save {
        name: String,
    },
    Run {
        name: String,
    },
    List {
        #[arg(long)]
        tag: Option<String>,
    },
    Show {
        name: String,
    },
    Copy {
        name: String,
    },
    Delete {
        name: String,
        #[arg(short, long)]
        force: bool,
    },
    Edit {
        name: String,
    },
    Export {
        path: String,
    },
}
