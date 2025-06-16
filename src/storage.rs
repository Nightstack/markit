use chrono::Utc;
use nucleo_matcher::{
    Config, Matcher,
    pattern::{CaseMatching, Normalization, Pattern},
};
use std::{fs, fs::File, io::Write, path::PathBuf};

use crate::models::{Snippet, SnippetStore};

pub fn save_to_file(new_snippet: Snippet) -> () {
    backup_current_store().unwrap();

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

pub fn get_snippets_by_name(query: &str) -> Option<SnippetStore> {
    let store = get_snippets()?;

    let names: Vec<&str> = store.snippets.iter().map(|s| s.name.as_str()).collect();
    let mut matcher = Matcher::new(Config::DEFAULT);

    let matches = Pattern::parse(query, CaseMatching::Ignore, Normalization::Smart)
        .match_list(&names, &mut matcher);

    let matched_snippets: Vec<Snippet> = matches
        .into_iter()
        .filter_map(|(matched_name, _)| {
            let matched_str = *matched_name;
            store
                .snippets
                .iter()
                .find(|s| s.name == matched_str)
                .cloned()
        })
        .collect();

    if matched_snippets.is_empty() {
        None
    } else {
        Some(SnippetStore {
            snippets: matched_snippets,
        })
    }
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
    backup_current_store()?;

    let path = get_storage_path();
    let file = File::create(&path)?;
    serde_yaml::to_writer(file, store)?;
    Ok(())
}

pub fn get_backup_files() -> Vec<PathBuf> {
    let backup_dir = get_storage_path().parent().unwrap().join("backups");

    if !backup_dir.exists() {
        return Vec::new();
    }

    let mut backups: Vec<PathBuf> = fs::read_dir(&backup_dir)
        .unwrap()
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.extension()?.to_str()? == "yml" {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    backups.sort_by(|a, b| b.file_name().cmp(&a.file_name()));
    backups
}

pub fn restore_backup(file_path: PathBuf) -> () {
    let storage_path = get_storage_path();

    match fs::copy(&file_path, &storage_path) {
        Ok(_) => println!("✅ Backup restored from '{}'", file_path.display()),
        Err(e) => eprintln!("⛔ Failed to restore backup: {}", e),
    }
}

fn get_storage_path() -> PathBuf {
    let dir = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".markit");
    std::fs::create_dir_all(&dir).unwrap();
    dir.join("bookmarks.yml")
}

fn backup_current_store() -> Result<(), Box<dyn std::error::Error>> {
    let store = get_snippets().ok_or("No data to back up")?;
    let backup_dir = get_storage_path().parent().unwrap().join("backups");
    fs::create_dir_all(&backup_dir)?;

    let timestamp = Utc::now().format("%Y-%m-%dT%H-%M-%SZ").to_string();
    let backup_file = backup_dir.join(format!("{}.yml", timestamp));
    let mut file = fs::File::create(backup_file)?;
    let yaml = serde_yaml::to_string(&store)?;
    file.write_all(yaml.as_bytes())?;

    Ok(())
}
