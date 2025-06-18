use std::{env, io::Write, process::Command};

use crate::{
    commands::helper::{get_snippet, redact_snippet},
    models::{PartialSnippet, Snippet},
    storage::Storage,
    ui::SelectionUI,
};
use tempfile::NamedTempFile;

pub fn edit_command(storage: &dyn Storage, selection_ui: &dyn SelectionUI, name: String) -> () {
    let mut store = match storage.load() {
        Ok(s) => s,
        Err(_) => {
            println!("📭 No snippets saved yet.");
            return;
        }
    };

    let mut original = match get_snippet(&store, selection_ui, name) {
        Some(s) => s,
        None => {
            return;
        }
    };

    let editable = redact_snippet(&original);

    let mut tmpfile = NamedTempFile::new().expect("⛔ Could not create temp file");
    let yaml = serde_yaml::to_string(&editable).expect("⛔ Could not serialise snippet");
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

    let edited: PartialSnippet = match std::fs::read_to_string(tmpfile.path())
        .ok()
        .and_then(|data| serde_yaml::from_str(&data).ok())
    {
        Some(s) => s,
        None => {
            eprintln!("⛔ Invalid YAML or missing fields. Edit aborted.");
            return;
        }
    };

    if store
        .snippets
        .iter()
        .any(|s| s.name.eq_ignore_ascii_case(&edited.name) && s.name != original.name)
    {
        eprintln!(
            "⛔ Another snippet with the name '{}' already exists.",
            edited.name
        );
        return;
    }

    store.snippets.retain(|s| s.name != original.name);
    apply_edits(&mut original, edited);
    store.snippets.push(original.clone());

    if let Err(err) = storage.save_all(&store) {
        eprintln!("⛔ Failed to update snippet: {:?}", err);
    } else {
        println!("✏️ Snippet '{}' updated.", original.name);
    }
}

fn apply_edits(original: &mut Snippet, edited: PartialSnippet) {
    original.name = edited.name;
    original.description = edited.description;
    original.content = edited.content;
    original.executable = edited.executable;
    original.updated_at = chrono::Utc::now();
    original.tags = edited.tags;
}
