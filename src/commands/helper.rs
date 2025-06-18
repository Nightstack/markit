use crate::{
    models::{PartialSnippet, Snippet, SnippetStore},
    storage::filter::{self, Filter},
    ui,
};

pub fn get_snippet(store: &SnippetStore, name: String) -> Option<Snippet> {
    let filtered = filter::apply_filter(&store, Filter::Name(name.clone()));

    return match ui::select_snippet(filtered) {
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
