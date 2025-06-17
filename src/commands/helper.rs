use crate::{
    models::{PartialSnippet, Snippet, SnippetStore},
    storage::filter::{self, Filter},
    ui::SelectionUI,
};

pub fn get_snippet(
    store: &SnippetStore,
    selection_ui: &dyn SelectionUI,
    name: String,
) -> Option<Snippet> {
    let filtered = filter::apply_filter(&store, Filter::Name(name.clone()));

    return match selection_ui.with_snippet_list(filtered) {
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
