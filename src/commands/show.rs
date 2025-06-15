use crate::storage;

pub fn show_command(name: String) -> () {
    match storage::get_snippet_by_name(&name) {
        Some(snippet) => {
            println!("🔎 Snippet: {}", snippet.name);
            println!("📄 Description: {}", snippet.description);
            println!("📋 Content:\n{}", snippet.content);
        }
        None => {
            println!("⛔ Snippet '{}' not found.", name);
        }
    }
}
