use nucleo_matcher::{
    Config, Matcher,
    pattern::{CaseMatching, Normalization, Pattern},
};
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

pub fn get_snippets_by_name(query: &str) -> Option<Vec<Snippet>> {
    let store = get_snippets()?;

    let names: Vec<&str> = store.snippets.iter().map(|s| s.name.as_str()).collect();
    let mut matcher = Matcher::new(Config::DEFAULT);

    let matches = Pattern::parse(query, CaseMatching::Ignore, Normalization::Smart)
        .match_list(&names, &mut matcher);

    matches
        .into_iter()
        .map(|(matched_name, _)| {
            let matched_str = *matched_name;
            store
                .snippets
                .iter()
                .find(|s| s.name == matched_str)
                .cloned()
        })
        .collect()
}

pub fn get_snippets_by_tag(tag: &str) -> Option<SnippetStore> {
    let store = get_snippets()?;

    let matching: Vec<Snippet> = store
        .snippets
        .iter()
        .filter(|s| s.tags.iter().any(|t| t.eq_ignore_ascii_case(tag)))
        .cloned()
        .collect();

    if matching.is_empty() {
        None
    } else {
        Some(SnippetStore { snippets: matching })
    }
}

pub fn write_snippets(store: &SnippetStore) -> Result<(), Box<dyn std::error::Error>> {
    let path = get_storage_path();
    let file = File::create(&path)?;
    serde_yaml::to_writer(file, store)?;
    Ok(())
}

fn get_storage_path() -> PathBuf {
    let dir = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".markit");
    std::fs::create_dir_all(&dir).unwrap();
    dir.join("bookmarks.yml")
}
