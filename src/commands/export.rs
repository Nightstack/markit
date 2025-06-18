use crate::{file::FileWriter, storage::Storage};

pub fn export_command(storage: &dyn Storage, writer: &dyn FileWriter, file_path: &str) {
    let store = match storage.load() {
        Ok(s) => s,
        Err(_) => {
            println!("📭 No snippets saved yet.");
            return;
        }
    };

    match writer.write_yaml(file_path, &store) {
        Ok(_) => println!("📦 Snippets exported to {file_path}"),
        Err(e) => eprintln!("⛔ Failed to export snippets: {e}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file::FileWriter;
    use crate::models::{Snippet, SnippetStore};
    use crate::storage::{Storage, StorageError};
    use std::cell::RefCell;

    struct MockStorage {
        snippets: Vec<Snippet>,
        should_fail: bool,
    }

    impl Storage for MockStorage {
        fn load(&self) -> Result<SnippetStore, StorageError> {
            if self.should_fail {
                Err(StorageError::Io(std::io::Error::other("Load failed")))
            } else {
                Ok(SnippetStore {
                    snippets: self.snippets.clone(),
                })
            }
        }

        fn save(&self, _: Snippet) -> Result<(), StorageError> {
            Ok(())
        }

        fn save_all(&self, _: &SnippetStore) -> Result<(), StorageError> {
            Ok(())
        }

        fn get_backups(&self) -> Result<Vec<std::path::PathBuf>, StorageError> {
            Ok(vec![])
        }

        fn restore_backup(&self, _: &std::path::Path) -> Result<(), StorageError> {
            Ok(())
        }
    }

    struct MockFileWriter {
        should_fail: bool,
        called_with: RefCell<Option<String>>, // Track what was passed
    }

    impl FileWriter for MockFileWriter {
        fn write_yaml(&self, file_path: &str, _store: &SnippetStore) -> Result<(), String> {
            self.called_with.replace(Some(file_path.to_string()));
            if self.should_fail {
                Err("Failed to write".to_string())
            } else {
                Ok(())
            }
        }
    }

    #[test]
    fn test_export_command_success() {
        let snippet = Snippet {
            name: "example".to_string(),
            description: "desc".to_string(),
            content: "echo hi".to_string(),
            executable: true,
            tags: vec!["tag".to_string()],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let storage = MockStorage {
            snippets: vec![snippet],
            should_fail: false,
        };

        let writer = MockFileWriter {
            should_fail: false,
            called_with: RefCell::new(None),
        };

        export_command(&storage, &writer, "output.yml");
        assert_eq!(writer.called_with.borrow().as_deref(), Some("output.yml"));
    }

    #[test]
    fn test_export_command_storage_failure() {
        let storage = MockStorage {
            snippets: vec![],
            should_fail: true,
        };

        let writer = MockFileWriter {
            should_fail: false,
            called_with: RefCell::new(None),
        };

        export_command(&storage, &writer, "output.yml");
        assert!(writer.called_with.borrow().is_none());
    }

    #[test]
    fn test_export_command_writer_failure() {
        let snippet = Snippet {
            name: "example".to_string(),
            description: "desc".to_string(),
            content: "echo hi".to_string(),
            executable: true,
            tags: vec!["tag".to_string()],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let storage = MockStorage {
            snippets: vec![snippet],
            should_fail: false,
        };

        let writer = MockFileWriter {
            should_fail: true,
            called_with: RefCell::new(None),
        };

        export_command(&storage, &writer, "output.yml");
        assert_eq!(writer.called_with.borrow().as_deref(), Some("output.yml"));
    }
}
