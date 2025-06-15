use crate::{storage, ui};

pub fn show_command(name: String) {
    let snippets = match storage::get_snippets_by_name(&name) {
        Some(s) => s,
        None => {
            println!("⛔ Snippet '{}' not found.", name);
            return;
        }
    };

    let snippet = match ui::select_snippet(snippets) {
        Some(s) => s,
        None => {
            println!("⛔ Snippet '{}' not found.", name);
            return;
        }
    };

    println!("🔎 Snippet: {}", snippet.name);
    println!("📄 Description: {}", snippet.description);
    println!("📋 Content:\n{}", snippet.content);
    println!("🚀 Executable: {}", snippet.executable);
}
