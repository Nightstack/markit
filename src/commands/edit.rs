use std::{env, io::Write, process::Command};

use crate::{commands::helper::get_snippet, models::Snippet, storage};
use tempfile::NamedTempFile;

pub fn edit_command(name: String) -> () {
    let mut store = match storage::get_snippets() {
        Some(s) => s,
        None => {
            println!("📭 No snippets to delete.");
            return;
        }
    };

    let original = match get_snippet(name) {
        Some(s) => s,
        None => {
            return;
        }
    };

    let mut tmpfile = NamedTempFile::new().expect("⛔ Could not create temp file");
    let yaml = serde_yaml::to_string(&original).expect("⛔ Could not serialise snippet");
    tmpfile
        .write_all(yaml.as_bytes())
        .expect("⛔ Could not write to temp file");

    let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());
    let status = Command::new(editor)
        .arg(tmpfile.path())
        .status()
        .expect("⛔ Failed to open editor");

    if !status.success() {
        eprintln!("⛔ Editor exited with an error.");
        return;
    }

    let edited: Snippet = match std::fs::read_to_string(tmpfile.path())
        .ok()
        .and_then(|data| serde_yaml::from_str(&data).ok())
    {
        Some(s) => s,
        None => {
            eprintln!("⛔ Invalid YAML or missing fields. Edit aborted.");
            return;
        }
    };

    store.snippets.retain(|s| s.name != original.name);
    store.snippets.push(edited.clone());

    if let Err(err) = storage::write_snippets(&store) {
        eprintln!("⛔ Failed to update snippet: {}", err);
    } else {
        println!("✏️ Snippet '{}' updated.", edited.name);
    }
}
