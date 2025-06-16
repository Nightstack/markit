use crate::{models::SnippetStore, storage};
use comfy_table::{Cell, Color, Row, Table, presets::UTF8_FULL};

pub fn list_command(tag: Option<String>) -> () {
    match tag {
        Some(t) => output_tagged_snippets(&t),
        None => output_all_snippets(),
    }
}

fn output_all_snippets() -> () {
    match storage::get_snippets() {
        Some(store) => {
            let table = build_output_table(store);
            println!("{table}");
        }
        None => {
            println!("📭 No snippets saved yet.");
        }
    }
}

fn output_tagged_snippets(tag: &str) -> () {
    match storage::get_snippets_by_tag(tag) {
        Some(store) => {
            let table = build_output_table(store);
            println!("{table}");
        }
        None => {
            println!("📭 No snippets found for tag: {}.", tag);
        }
    }
}

fn build_output_table(store: SnippetStore) -> Table {
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

    for snippet in store.snippets {
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
