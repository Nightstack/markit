use comfy_table::Table;

use crate::models::Snippet;

pub mod cli_selection;
pub mod cli_table;

pub trait TableUI {
    fn with_snippet_list(&mut self, snippets: Vec<Snippet>) -> Table;
}

pub trait SelectionUI {
    fn with_snippet_list(&self, snippets: Vec<Snippet>) -> Option<Snippet>;
    fn with_backup_list(&self, backups: &[String]) -> Option<usize>;
}
