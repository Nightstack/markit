use dialoguer::{Select, theme::ColorfulTheme};

use crate::models::Snippet;

pub fn select_snippet(matches: Vec<Snippet>) -> Option<Snippet> {
    if matches.len() == 1 {
        return matches.get(0).cloned();
    }

    let options: Vec<&str> = matches.iter().map(|s| s.name.as_str()).collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Multiple matches found. Select one:")
        .items(&options)
        .default(0)
        .interact()
        .ok()?;

    matches.get(selection).cloned()
}

pub fn select_backup(backups: &[String]) -> Option<usize> {
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt("📦 Select a backup to restore:")
        .items(backups)
        .default(0)
        .interact()
        .ok()
}
