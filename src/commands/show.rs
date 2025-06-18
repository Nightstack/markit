use crate::{commands::helper::get_snippet, storage::Storage};

pub fn show_command(storage: &dyn Storage, name: String) {
    let store = match storage.load() {
        Ok(s) => s,
        Err(_) => {
            println!("📭 No snippets saved yet.");
            return;
        }
    };

    let snippet = get_snippet(&store, name).unwrap();

    println!("🔎 Snippet: {}", snippet.name);
    println!("📄 Description: {}", snippet.description);
    println!("🚀 Executable: {}", snippet.executable);
    println!("🕒 Created at: {}", snippet.created_at);
    println!("🕒 Updated at: {}", snippet.updated_at);
    println!("📋 Content:\n{}", snippet.content);
    println!("🏷️ Tags: {}", snippet.tags.join(", "));
}
