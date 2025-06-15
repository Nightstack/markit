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
    Save { name: String },
    Run { name: String },
    List,
    Show { name: String },
    Copy { name: String },
    Delete { name: String },
}
