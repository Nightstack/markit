use arboard::Clipboard;

use crate::{commands::helper::get_snippet, storage::Storage, ui::SelectionUI};

pub fn copy_command(storage: &dyn Storage, selection_ui: &dyn SelectionUI, name: String) -> () {
    let store = match storage.load() {
        Ok(s) => s,
        Err(_) => {
            println!("📭 No snippets saved yet.");
            return;
        }
    };

    let snippet = get_snippet(&store, selection_ui, name).unwrap();

    let mut clipboard = Clipboard::new().expect("Failed to access clipboard");
    clipboard
        .set_text(snippet.content.clone())
        .expect("Failed to copy to clipboard");
    println!("📋 Snippet '{}' copied to clipboard", snippet.name);
}
