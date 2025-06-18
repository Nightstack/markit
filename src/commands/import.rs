use crate::{file::FileReader, storage::Storage};

pub fn import_command(storage: &dyn Storage, reader: &dyn FileReader, file_path: &str) {
    let imported = match reader.read_yaml(file_path) {
        Ok(store) => store,
        Err(e) => {
            eprintln!("⛔ Failed to read import file: {}", e);
            return;
        }
    };

    let mut store = match storage.load() {
        Ok(s) => s,
        Err(_) => {
            println!("📭 No snippets saved yet.");
            return;
        }
    };

    let mut added = 0;
    for snippet in imported.snippets {
        if !store.snippets.iter().any(|s| s.name == snippet.name) {
            store.snippets.push(snippet);
            added += 1;
        }
    }

    if let Err(err) = storage.save_all(&store) {
        eprintln!("⛔ Failed to update storage: {:?}", err);
    } else {
        println!("📥 Imported {added} new snippet(s) from {file_path}");
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        file::FileReader,
        import::import_command,
        models::{Snippet, SnippetStore},
        storage::{Storage, StorageError},
    };
    use chrono::Utc;
    use std::cell::RefCell;

    struct MockStorage {
        store: RefCell<SnippetStore>,
        fail_load: bool,
        fail_save: bool,
        save_calls: RefCell<u32>,
    }

    impl Storage for MockStorage {
        fn load(&self) -> Result<SnippetStore, StorageError> {
            if self.fail_load {
                Err(StorageError::Io(std::io::Error::other("Load failed")))
            } else {
                Ok(self.store.borrow().clone())
            }
        }

        fn save(&self, _: Snippet) -> Result<(), StorageError> {
            Ok(())
        }

        fn save_all(&self, store: &SnippetStore) -> Result<(), StorageError> {
            *self.store.borrow_mut() = store.clone();
            *self.save_calls.borrow_mut() += 1;
            if self.fail_save {
                Err(StorageError::Io(std::io::Error::other("Save failed")))
            } else {
                Ok(())
            }
        }

        fn get_backups(&self) -> Result<Vec<std::path::PathBuf>, StorageError> {
            Ok(vec![])
        }

        fn restore_backup(&self, _: &std::path::Path) -> Result<(), StorageError> {
            Ok(())
        }
    }

    struct MockFileReader {
        should_fail: bool,
        store: SnippetStore,
    }

    impl FileReader for MockFileReader {
        fn read_yaml(&self, _path: &str) -> Result<SnippetStore, StorageError> {
            if self.should_fail {
                Err(StorageError::Io(std::io::Error::other("mock error")))
            } else {
                Ok(self.store.clone())
            }
        }
    }

    fn test_snippet(name: &str) -> Snippet {
        Snippet {
            name: name.to_string(),
            description: "desc".into(),
            content: "echo hi".into(),
            executable: false,
            tags: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn test_import_success() {
        let initial = SnippetStore {
            snippets: vec![test_snippet("a")],
        };

        let imported = SnippetStore {
            snippets: vec![test_snippet("a"), test_snippet("b")],
        };

        let storage = MockStorage {
            store: RefCell::new(initial),
            fail_load: false,
            fail_save: false,
            save_calls: RefCell::new(0),
        };

        let reader = MockFileReader {
            should_fail: false,
            store: imported,
        };

        import_command(&storage, &reader, "test.yml");
        assert_eq!(storage.store.borrow().snippets.len(), 2);
        assert_eq!(*storage.save_calls.borrow(), 1);
    }

    #[test]
    fn test_import_file_read_failure() {
        let storage = MockStorage {
            store: RefCell::new(SnippetStore::default()),
            fail_load: false,
            fail_save: false,
            save_calls: RefCell::new(0),
        };

        let reader = MockFileReader {
            should_fail: true,
            store: SnippetStore::default(), // safe fallback
        };

        import_command(&storage, &reader, "nonexistent.yml");
        assert_eq!(*storage.save_calls.borrow(), 0);
    }

    #[test]
    fn test_import_storage_load_failure() {
        let imported = SnippetStore {
            snippets: vec![test_snippet("new")],
        };

        let storage = MockStorage {
            store: RefCell::new(SnippetStore::default()),
            fail_load: true,
            fail_save: false,
            save_calls: RefCell::new(0),
        };

        let reader = MockFileReader {
            should_fail: false,
            store: imported,
        };

        import_command(&storage, &reader, "test.yml");
        assert_eq!(*storage.save_calls.borrow(), 0);
    }

    #[test]
    fn test_import_no_new_snippets() {
        let snippet = test_snippet("a");

        let imported = SnippetStore {
            snippets: vec![snippet.clone()],
        };

        let storage = MockStorage {
            store: RefCell::new(SnippetStore {
                snippets: vec![snippet],
            }),
            fail_load: false,
            fail_save: false,
            save_calls: RefCell::new(0),
        };

        let reader = MockFileReader {
            should_fail: false,
            store: imported,
        };

        import_command(&storage, &reader, "test.yml");
        assert_eq!(storage.store.borrow().snippets.len(), 1);
        assert_eq!(*storage.save_calls.borrow(), 1);
    }

    #[test]
    fn test_import_save_all_failure() {
        let imported = SnippetStore {
            snippets: vec![test_snippet("new")],
        };

        let storage = MockStorage {
            store: RefCell::new(SnippetStore::default()),
            fail_load: false,
            fail_save: true,
            save_calls: RefCell::new(0),
        };

        let reader = MockFileReader {
            should_fail: false,
            store: imported,
        };

        import_command(&storage, &reader, "test.yml");
        assert_eq!(storage.store.borrow().snippets.len(), 1);
        assert_eq!(*storage.save_calls.borrow(), 1);
    }
}
