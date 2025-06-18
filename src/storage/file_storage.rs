use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use chrono::Utc;

use crate::{
    models::{Snippet, SnippetStore},
    storage::{Storage, StorageError},
};

pub struct FileStorage {
    base_path: PathBuf,
}

impl FileStorage {
    pub fn new() -> Self {
        let base_path = dirs::home_dir()
            .unwrap_or_else(|| {
                eprintln!(
                    "⚠️ Could not determine home directory, defaulting to current directory."
                );
                PathBuf::from(".")
            })
            .join(".markit");

        if let Err(e) = fs::create_dir_all(&base_path) {
            eprintln!("⛔ Failed to create base directory: {}", e);
        }

        Self { base_path }
    }

    fn storage_path(&self) -> PathBuf {
        self.base_path.join("bookmarks.yml")
    }

    fn backup_dir(&self) -> PathBuf {
        self.base_path.join("backups")
    }

    fn load_store(&self) -> Result<SnippetStore, StorageError> {
        let path = self.storage_path();
        if !path.exists() {
            return Ok(SnippetStore::default());
        }

        let file = File::open(&path)?;
        let store = serde_yaml::from_reader(file)?;
        Ok(store)
    }

    fn backup_current_store(&self, store: &SnippetStore) -> Result<(), StorageError> {
        let backup_dir = self.backup_dir();
        fs::create_dir_all(&backup_dir).map_err(StorageError::Io)?;

        let timestamp = Utc::now().format("%Y-%m-%dT%H-%M-%SZ").to_string();
        let backup_file = backup_dir.join(format!("{}.yml", timestamp));
        let mut file = File::create(backup_file).map_err(StorageError::Io)?;
        let yaml = serde_yaml::to_string(store).map_err(StorageError::Serde)?;
        file.write_all(yaml.as_bytes()).map_err(StorageError::Io)?;

        Ok(())
    }
}

impl Storage for FileStorage {
    fn load(&self) -> Result<SnippetStore, StorageError> {
        self.load_store()
    }

    fn save(&self, snippet: Snippet) -> Result<(), StorageError> {
        let mut store = self.load_store()?;
        self.backup_current_store(&store)?;

        store.snippets.push(snippet);

        let file = File::create(self.storage_path()).map_err(StorageError::Io)?;
        serde_yaml::to_writer(file, &store).map_err(StorageError::Serde)?;

        println!("✅ Snippet saved.");
        Ok(())
    }

    fn save_all(&self, store: &SnippetStore) -> Result<(), StorageError> {
        self.backup_current_store(&self.load_store()?)?;

        let file = File::create(self.storage_path()).map_err(StorageError::Io)?;
        serde_yaml::to_writer(file, store).map_err(StorageError::Serde)?;

        Ok(())
    }

    fn get_backups(&self) -> Result<Vec<PathBuf>, StorageError> {
        let backup_dir = self.backup_dir();

        let mut backups: Vec<PathBuf> = fs::read_dir(&backup_dir)
            .map_err(StorageError::Io)?
            .filter_map(|entry| {
                let path = entry.ok()?.path();
                if path.extension()?.to_str()? == "yml" {
                    Some(path)
                } else {
                    None
                }
            })
            .collect();

        backups.sort_by(|a, b| b.file_name().cmp(&a.file_name()));
        Ok(backups)
    }

    fn restore_backup(&self, path: &Path) -> Result<(), StorageError> {
        fs::copy(path, self.storage_path())
            .map_err(StorageError::Io)
            .map(|_| {
                println!("✅ Backup restored from '{}'", path.display());
            })
    }
}
