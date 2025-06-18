use crate::storage::Storage;
use crate::ui::SelectionUI;

pub fn restore_command(storage: &dyn Storage, selection_ui: &dyn SelectionUI) {
    let backups = match storage.get_backups() {
        Ok(s) => s,
        Err(_) => {
            println!("📭 No backups created yet.");
            return;
        }
    };

    if backups.is_empty() {
        println!("📭 No backups found.");
        return;
    }

    let display_names: Vec<String> = backups
        .iter()
        .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
        .collect();

    let selected_index = match selection_ui.with_backup_list(&display_names) {
        Some(i) => i,
        None => return,
    };

    let full_path = backups.get(selected_index).unwrap().clone();

    match storage.restore_backup(&full_path) {
        Ok(_) => println!("✅ Backup restored successfully."),
        Err(e) => eprintln!("⛔ Failed to restore backup: {}", e),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::Snippet,
        storage::{Storage, StorageError},
        ui::SelectionUI,
    };
    use std::{cell::RefCell, path::PathBuf};

    struct MockStorage {
        backups: Vec<PathBuf>,
        restore_called_with: RefCell<Option<PathBuf>>,
        fail_get: bool,
        fail_restore: bool,
    }

    impl Storage for MockStorage {
        fn load(&self) -> Result<crate::models::SnippetStore, StorageError> {
            Ok(crate::models::SnippetStore::default())
        }

        fn save(&self, _: Snippet) -> Result<(), StorageError> {
            Ok(())
        }

        fn save_all(&self, _: &crate::models::SnippetStore) -> Result<(), StorageError> {
            Ok(())
        }

        fn get_backups(&self) -> Result<Vec<PathBuf>, StorageError> {
            if self.fail_get {
                Err(StorageError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to get backups",
                )))
            } else {
                Ok(self.backups.clone())
            }
        }

        fn restore_backup(&self, path: &PathBuf) -> Result<(), StorageError> {
            self.restore_called_with.replace(Some(path.clone()));
            if self.fail_restore {
                Err(StorageError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to restore",
                )))
            } else {
                Ok(())
            }
        }
    }

    struct MockSelectionUI {
        selected_index: Option<usize>,
    }

    impl SelectionUI for MockSelectionUI {
        fn with_snippet_list(&self, _snippets: Vec<Snippet>) -> Option<Snippet> {
            None
        }

        fn with_backup_list(&self, _backups: &[String]) -> Option<usize> {
            self.selected_index
        }
    }

    #[test]
    fn test_restore_success() {
        let path = PathBuf::from("backup1.yml");

        let storage = MockStorage {
            backups: vec![path.clone()],
            restore_called_with: RefCell::new(None),
            fail_get: false,
            fail_restore: false,
        };

        let ui = MockSelectionUI {
            selected_index: Some(0),
        };

        restore_command(&storage, &ui);

        assert_eq!(*storage.restore_called_with.borrow(), Some(path));
    }

    #[test]
    fn test_no_backups_found() {
        let storage = MockStorage {
            backups: vec![],
            restore_called_with: RefCell::new(None),
            fail_get: false,
            fail_restore: false,
        };

        let ui = MockSelectionUI {
            selected_index: Some(0),
        };

        restore_command(&storage, &ui);

        assert!(storage.restore_called_with.borrow().is_none());
    }

    #[test]
    fn test_user_cancels_selection() {
        let storage = MockStorage {
            backups: vec![PathBuf::from("backup.yml")],
            restore_called_with: RefCell::new(None),
            fail_get: false,
            fail_restore: false,
        };

        let ui = MockSelectionUI {
            selected_index: None,
        };

        restore_command(&storage, &ui);

        assert!(storage.restore_called_with.borrow().is_none());
    }

    #[test]
    fn test_get_backups_fails() {
        let storage = MockStorage {
            backups: vec![],
            restore_called_with: RefCell::new(None),
            fail_get: true,
            fail_restore: false,
        };

        let ui = MockSelectionUI {
            selected_index: Some(0),
        };

        restore_command(&storage, &ui);

        assert!(storage.restore_called_with.borrow().is_none());
    }

    #[test]
    fn test_restore_fails() {
        let path = PathBuf::from("broken_backup.yml");

        let storage = MockStorage {
            backups: vec![path.clone()],
            restore_called_with: RefCell::new(None),
            fail_get: false,
            fail_restore: true,
        };

        let ui = MockSelectionUI {
            selected_index: Some(0),
        };

        restore_command(&storage, &ui);

        // Called but failed internally
        assert_eq!(*storage.restore_called_with.borrow(), Some(path));
    }
}
