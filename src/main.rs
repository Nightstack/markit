mod cli;
use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Save { name } => {
            println!("Would save command '{}'", name);
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
