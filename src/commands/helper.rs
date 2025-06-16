use crate::{
    models::{PartialSnippet, Snippet},
    storage, ui,
};

pub fn get_snippet(name: String) -> Option<Snippet> {
    let store = match storage::get_snippets_by_name(&name) {
        Some(s) => s,
        None => {
            println!("⛔ Snippet '{}' not found.", name);
            return None;
        }
    };

    return match ui::select_snippet(store.snippets) {
        Some(s) => Some(s),
        None => {
            println!("⛔ Snippet '{}' not found.", name);
            return None;
        }
    };
}

pub fn redact_snippet(snippet: &Snippet) -> PartialSnippet {
    PartialSnippet {
        name: snippet.name.clone(),
        description: snippet.description.clone(),
        content: snippet.content.clone(),
        executable: snippet.executable,
        tags: snippet.tags.clone(),
    }
}
