use crate::{
    models::Snippet,
    storage::{
        Storage,
        filter::{Filter, apply_filter},
    },
    ui::TableUI,
};

pub fn list_command(storage: &dyn Storage, table_ui: &mut dyn TableUI, tag: Option<String>) -> () {
    let store = match storage.load() {
        Ok(s) => s,
        Err(_) => {
            println!("📭 No snippets saved yet.");
            return;
        }
    };

    let snippets: Vec<Snippet> = match tag.as_deref() {
        Some(tag) => apply_filter(&store, Filter::Tag(tag.to_string())),
        None => apply_filter(&store, Filter::All),
    };

    if snippets.is_empty() {
        if let Some(tag) = tag {
            println!("📭 No snippets found for tag: {}.", tag);
        } else {
            println!("📭 No snippets saved yet.");
        }
    } else {
        let table = table_ui.with_snippet_list(snippets);
        println!("{table}");
    }
}
