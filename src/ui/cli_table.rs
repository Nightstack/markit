use crate::{models::Snippet, ui::TableUI};
use comfy_table::{Cell, Color, Row, Table, presets::UTF8_FULL};

pub struct CliTable {
    table: Table,
    header_color: Color,
}

impl CliTable {
    pub fn new() -> Self {
        let mut table = Table::new();
        table.load_preset(UTF8_FULL);

        let header_color = Color::Rgb {
            r: 100,
            g: 255,
            b: 255,
        };

        Self {
            table,
            header_color,
        }
    }
}

impl TableUI for CliTable {
    fn with_snippet_list(&mut self, snippets: Vec<Snippet>) -> Table {
        self.table.set_header(vec![
            Cell::new("Name").fg(self.header_color),
            Cell::new("Description").fg(self.header_color),
            Cell::new("Executable").fg(self.header_color),
            Cell::new("Created at").fg(self.header_color),
            Cell::new("Updated at").fg(self.header_color),
            Cell::new("Tags").fg(self.header_color),
        ]);

        for snippet in snippets {
            self.table.add_row(Row::from(vec![
                Cell::new(snippet.name).fg(Color::White),
                Cell::new(snippet.description).fg(Color::White),
                Cell::new(if snippet.executable { "yes" } else { "no" }).fg(Color::White),
                Cell::new(snippet.created_at).fg(Color::White),
                Cell::new(snippet.updated_at).fg(Color::White),
                Cell::new(snippet.tags.join(", ")).fg(Color::White),
            ]));
        }

        self.table.clone()
    }
}
