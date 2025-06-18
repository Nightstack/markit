use crate::{file::FileReader, models::SnippetStore, storage::StorageError};
use std::{fs::File, path::Path};

pub struct Reader;

impl FileReader for Reader {
    fn read_yaml(&self, path: &str) -> Result<SnippetStore, StorageError> {
        let file = File::open(Path::new(path)).map_err(StorageError::Io)?;
        let store = serde_yaml::from_reader(file).map_err(StorageError::Serde)?;
        Ok(store)
    }
}
