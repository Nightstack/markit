use crate::{models::Snippet, storage, ui};

pub fn get_snippet(name: String) -> Option<Snippet> {
    let snippets = match storage::get_snippets_by_name(&name) {
        Some(s) => s,
        None => {
            println!("⛔ Snippet '{}' not found.", name);
            return None;
        }
    };

    return match ui::select_snippet(snippets) {
        Some(s) => Some(s),
        None => {
            println!("⛔ Snippet '{}' not found.", name);
            return None;
        }
    };
}
