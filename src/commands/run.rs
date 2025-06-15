use crate::{storage, ui};
use std::process::Command;

pub fn run_command(name: String) -> () {
    let snippets = match storage::get_snippets_by_name(&name) {
        Some(s) => s,
        None => {
            println!("⛔ Snippet '{}' not found.", name);
            return;
        }
    };

    let snippet = match ui::select_snippet(snippets) {
        Some(s) => s,
        None => {
            println!("⛔ Snippet '{}' not found.", name);
            return;
        }
    };

    println!("🚀 Running: {}", snippet.name);
    println!("📋 {}", snippet.content);

    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".into());
    let status = Command::new(shell).arg("-c").arg(&snippet.content).status();

    match status {
        Ok(code) if code.success() => println!("✅ Command ran successfully."),
        Ok(code) => println!("⚠️ Command exited with status: {}", code),
        Err(err) => println!("⛔ Failed to run command: {}", err),
    }
}
