use crate::{file::FileWriter, models::SnippetStore};
use std::{fs::File, path::Path};

pub struct Writer;

impl FileWriter for Writer {
    fn write_yaml(&self, path: &str, store: &SnippetStore) -> Result<(), String> {
        match File::create(Path::new(path)) {
            Ok(file) => serde_yaml::to_writer(file, store).map_err(|e| e.to_string()),
            Err(err) => Err(err.to_string()),
        }
    }
}
