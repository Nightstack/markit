use dialoguer::Confirm;

use crate::{commands::helper::get_snippet, storage::Storage};

pub fn delete_command(storage: &dyn Storage, name: String, force: bool) -> () {
    let mut store = match storage.load() {
        Ok(s) => s,
        Err(_) => {
            println!("📭 No snippets saved yet.");
            return;
        }
    };

    let delete_snippet = match get_snippet(&store, name) {
        Some(s) => s,
        None => {
            return;
        }
    };

    if !force {
        let prompt = format!(
            "❗ Are you sure you want to delete '{}'? This cannot be undone.",
            delete_snippet.name
        );
        if !Confirm::new()
            .with_prompt(prompt)
            .default(false)
            .interact()
            .unwrap_or(false)
        {
            println!("🚫 Deletion cancelled.");
            return;
        }
    }

    store.snippets.retain(|s| s.name != delete_snippet.name);

    if let Err(err) = storage.save_all(&store) {
        eprintln!("⛔ Failed to update snippets file: {:?}", err);
    } else {
        println!("🗑️ Snippet '{}' deleted.", delete_snippet.name);
    }
}
