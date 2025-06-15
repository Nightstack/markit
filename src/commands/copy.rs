use arboard::Clipboard;

use crate::{storage, ui};

pub fn copy_command(name: String) -> () {
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

    let mut clipboard = Clipboard::new().expect("Failed to access clipboard");
    clipboard
        .set_text(snippet.content.clone())
        .expect("Failed to copy to clipboard");
    println!("📋 Snippet '{}' copied to clipboard", snippet.name);
}
