use crate::{commands::helper::get_snippet, storage};

pub fn delete_command(name: String) -> () {
    let mut store = match storage::get_snippets() {
        Some(s) => s,
        None => {
            println!("📭 No snippets to delete.");
            return;
        }
    };

    let delete_snippet = match get_snippet(name) {
        Some(s) => s,
        None => {
            return;
        }
    };

    store.snippets.retain(|s| s.name != delete_snippet.name);

    if let Err(err) = storage::write_snippets(&store) {
        eprintln!("⛔ Failed to update snippets file: {}", err);
    } else {
        println!("🗑️ Snippet '{}' deleted.", delete_snippet.name);
    }
}
