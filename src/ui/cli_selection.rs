use dialoguer::{Select, theme::ColorfulTheme};

use crate::{models::Snippet, ui::SelectionUI};

pub struct CliSelection {}

impl CliSelection {
    pub fn new() -> Self {
        Self {}
    }

    fn snippet_prompt(&self) -> String {
        "📋 Select a snippet:".to_string()
    }
    fn backup_prompt(&self) -> String {
        "📦 Select a backup to restore:".to_string()
    }
}

impl SelectionUI for CliSelection {
    fn with_snippet_list(&self, snippets: Vec<Snippet>) -> Option<Snippet> {
        if snippets.len() == 1 {
            return snippets.get(0).cloned();
        }

        let options: Vec<&str> = snippets.iter().map(|s| s.name.as_str()).collect();

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(self.snippet_prompt())
            .items(&options)
            .default(0)
            .interact()
            .ok()?;

        snippets.get(selection).cloned()
    }

    fn with_backup_list(&self, backups: &[String]) -> Option<usize> {
        Select::with_theme(&ColorfulTheme::default())
            .with_prompt(self.backup_prompt())
            .items(backups)
            .default(0)
            .interact()
            .ok()
    }
}
