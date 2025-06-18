use crate::models::{PartialSnippet, SnippetStore};
use crate::storage::StorageError;

pub mod editor;
pub mod reader;
pub mod writer;

pub trait EditorLauncher {
    fn open_editor(&self, snippet: &PartialSnippet) -> Result<PartialSnippet, String>;
}

pub trait FileWriter {
    fn write_yaml(&self, path: &str, store: &SnippetStore) -> Result<(), String>;
}

pub trait FileReader {
    fn read_yaml(&self, path: &str) -> Result<SnippetStore, StorageError>;
}
