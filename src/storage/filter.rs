use crate::models::{Snippet, SnippetStore};

pub enum Filter {
    All,
    Name(String),
    Tag(String),
}

pub fn apply_filter(store: &SnippetStore, filter: Filter) -> Vec<Snippet> {
    match filter {
        Filter::All => store.snippets.clone(),
        Filter::Name(name) => get_by_name(store, &name),
        Filter::Tag(tag) => get_by_tag(store, &tag),
    }
}

fn get_by_name(store: &SnippetStore, name: &str) -> Vec<Snippet> {
    store
        .snippets
        .iter()
        .filter(|s| s.name.to_lowercase().contains(&name.to_lowercase()))
        .cloned()
        .collect()
}

fn get_by_tag(store: &SnippetStore, tag: &str) -> Vec<Snippet> {
    store
        .snippets
        .iter()
        .filter(|s| {
            s.tags
                .iter()
                .any(|t| t.to_lowercase() == tag.to_lowercase())
        })
        .cloned()
        .collect()
}
