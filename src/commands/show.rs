use crate::{models::Snippet, storage};
use dialoguer::{Select, theme::ColorfulTheme};

pub fn show_command(name: String) {
    let snippets = match storage::get_snippets_by_name(&name) {
        Some(s) => s,
        None => {
            println!("⛔ Snippet '{}' not found.", name);
            return;
        }
    };

    let snippet = match select_snippet(snippets) {
        Some(s) => s,
        None => {
            println!("⛔ Snippet '{}' not found.", name);
            return;
        }
    };

    println!("🔎 Snippet: {}", snippet.name);
    println!("📄 Description: {}", snippet.description);
    println!("📋 Content:\n{}", snippet.content);
}

fn select_snippet(matches: Vec<Snippet>) -> Option<Snippet> {
    let options: Vec<&str> = matches.iter().map(|s| s.name.as_str()).collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Multiple matches found. Select one:")
        .items(&options)
        .default(0)
        .interact()
        .ok()?;

    matches.get(selection).cloned()
}
