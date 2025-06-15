use crate::storage;

pub fn list_command() -> () {
    match storage::get_snippets() {
        Some(store) => {
            println!("📚 Saved Snippets:\n");

            for snippet in store.snippets {
                println!("🔹 {}\n   {}", snippet.name, snippet.description);
            }
        }
        None => {
            println!("📭 No snippets saved yet.");
        }
    }
}
