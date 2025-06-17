use crate::storage::Storage;
use std::{fs::File, path::Path};

pub fn export_command(storage: &dyn Storage, file_path: &str) {
    let store = match storage.load() {
        Ok(s) => s,
        Err(_) => {
            println!("📭 No snippets saved yet.");
            return;
        }
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
