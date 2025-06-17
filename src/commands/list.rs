use crate::{
    models::Snippet,
    storage::{
        Storage,
        filter::{Filter, apply_filter},
    },
};
use comfy_table::{Cell, Color, Row, Table, presets::UTF8_FULL};

pub fn list_command(storage: &dyn Storage, tag: Option<String>) -> () {
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
        let table = build_output_table(snippets);
        println!("{table}");
    }
}

fn build_output_table(snippets: Vec<Snippet>) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    let header_color = Color::Rgb {
        r: 100,
        g: 255,
        b: 255,
    };

    // Set colored header
    table.set_header(vec![
        Cell::new("Name").fg(header_color),
        Cell::new("Description").fg(header_color),
        Cell::new("Executable").fg(header_color),
        Cell::new("Created at").fg(header_color),
        Cell::new("Updated at").fg(header_color),
        Cell::new("Tags").fg(header_color),
    ]);

    for snippet in snippets {
        table.add_row(Row::from(vec![
            Cell::new(snippet.name).fg(Color::White),
            Cell::new(snippet.description).fg(Color::White),
            Cell::new(if snippet.executable { "yes" } else { "no" }).fg(Color::White),
            Cell::new(snippet.created_at).fg(Color::White),
            Cell::new(snippet.updated_at).fg(Color::White),
            Cell::new(snippet.tags.join(", ")).fg(Color::White),
        ]));
    }

    table
}
