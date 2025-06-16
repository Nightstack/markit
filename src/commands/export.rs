use crate::storage;
use std::{fs::File, path::Path};

pub fn export_command(file_path: &str) {
    let Some(store) = storage::get_snippets() else {
        println!("📭 No snippets to export.");
        return;
    };

    match File::create(Path::new(file_path)) {
        Ok(file) => {
            if let Err(err) = serde_yaml::to_writer(file, &store) {
                eprintln!("⛔ Failed to write YAML: {err}");
            } else {
                println!("📦 Snippets exported to {file_path}");
            }
        }
        Err(err) => eprintln!("⛔ Failed to create export file: {err}"),
    }
}
