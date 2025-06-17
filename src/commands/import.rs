use crate::{models::SnippetStore, storage::Storage};
use std::{fs::File, path::Path};

pub fn import_command(storage: &dyn Storage, file_path: &str) {
    let file = match File::open(Path::new(file_path)) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("⛔ Failed to open import file: {err}");
            return;
        }
    };

    let imported: SnippetStore = match serde_yaml::from_reader(file) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("⛔ Failed to parse YAML: {err}");
            return;
        }
    };

    let mut store = match storage.load() {
        Ok(s) => s,
        Err(_) => {
            println!("📭 No snippets saved yet.");
            return;
        }
    };

    let mut added = 0;
    for snippet in imported.snippets {
        if !store.snippets.iter().any(|s| s.name == snippet.name) {
            store.snippets.push(snippet);
            added += 1;
        }
    }

    if let Err(err) = storage.save_all(&store) {
        eprintln!("⛔ Failed to update storage: {:?}", err);
    } else {
        println!("📥 Imported {added} new snippet(s) from {file_path}");
    }
}
