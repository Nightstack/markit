use std::{fs::File, path::PathBuf};

use crate::models::{Snippet, SnippetStore};

pub fn save_to_file(new_snippet: Snippet) -> () {
    let path = get_storage_path();

    let mut store = if path.exists() {
        let file = File::open(&path).unwrap();
        serde_yaml::from_reader(file).unwrap_or_default()
    } else {
        SnippetStore::default()
    };

    store.snippets.push(new_snippet);

    let file = File::create(&path).unwrap();
    serde_yaml::to_writer(file, &store).unwrap();

    println!("✅ Snippet saved.");
}

pub fn get_snippets() -> Option<SnippetStore> {
    let path = get_storage_path();

    if !path.exists() {
        return None;
    }

    let file = File::open(&path).unwrap();
    let store: SnippetStore = serde_yaml::from_reader(file).unwrap_or_default();

    if store.snippets.is_empty() {
        return None;
    }

    Some(store)
}

fn get_storage_path() -> PathBuf {
    let dir = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".markit");
    std::fs::create_dir_all(&dir).unwrap();
    dir.join("bookmarks.yml")
}
