use crate::{commands::helper::get_snippet, storage::Storage};
use std::process::Command;

pub fn run_command(storage: &dyn Storage, name: String) -> () {
    let store = match storage.load() {
        Ok(s) => s,
        Err(_) => {
            println!("📭 No snippets saved yet.");
            return;
        }
    };

    let snippet = get_snippet(&store, name).unwrap();

    if !snippet.executable {
        println!("⛔ Snippet '{}' not executable.", snippet.name);
        return;
    }

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
