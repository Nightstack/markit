use std::path::PathBuf;
use std::{fmt, path::Path};

use crate::models::{Snippet, SnippetStore};

pub mod file_storage;
pub mod filter;

#[derive(Debug)]
pub enum StorageError {
    Io(std::io::Error),
    Serde(serde_yaml::Error),
}

impl From<std::io::Error> for StorageError {
    fn from(e: std::io::Error) -> Self {
        StorageError::Io(e)
    }
}

impl From<serde_yaml::Error> for StorageError {
    fn from(e: serde_yaml::Error) -> Self {
        StorageError::Serde(e)
    }
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageError::Io(e) => write!(f, "IO error: {}", e),
            StorageError::Serde(e) => write!(f, "Serialization error: {}", e),
        }
    }
}

pub trait Storage {
    fn load(&self) -> Result<SnippetStore, StorageError>;
    fn save(&self, snippet: Snippet) -> Result<(), StorageError>;
    fn save_all(&self, store: &SnippetStore) -> Result<(), StorageError>;
    fn get_backups(&self) -> Result<Vec<PathBuf>, StorageError>;
    fn restore_backup(&self, path: &Path) -> Result<(), StorageError>;
}
