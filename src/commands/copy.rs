use crate::{
    clipboard_provider::ClipboardProvider, commands::helper::get_snippet, storage::Storage,
    ui::SelectionUI,
};

pub fn copy_command(
    storage: &dyn Storage,
    selection_ui: &dyn SelectionUI,
    clipboard: &mut dyn ClipboardProvider,
    name: String,
) -> () {
    let store = match storage.load() {
        Ok(s) => s,
        Err(_) => {
            println!("📭 No snippets saved yet.");
            return;
        }
    };

    let Some(snippet) = get_snippet(&store, selection_ui, name) else {
        return;
    };

    if let Err(e) = clipboard.set_text(snippet.content.clone()) {
        eprintln!("⛔ Failed to copy to clipboard: {}", e);
        return;
    }

    println!("📋 Snippet '{}' copied to clipboard", snippet.name);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clipboard_provider::ClipboardProvider;
    use crate::models::{Snippet, SnippetStore};
    use crate::storage::{Storage, StorageError};
    use crate::ui::SelectionUI;
    use std::cell::RefCell;

    struct MockStorage {
        snippets: Vec<Snippet>,
        should_fail: bool,
    }

    impl Storage for MockStorage {
        fn load(&self) -> Result<SnippetStore, StorageError> {
            if self.should_fail {
                Err(StorageError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Load failed",
                )))
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

        fn restore_backup(&self, _: &std::path::PathBuf) -> Result<(), StorageError> {
            Ok(())
        }
    }

    struct MockSelectionUI {
        selected: RefCell<Option<Snippet>>,
    }

    impl SelectionUI for MockSelectionUI {
        fn with_snippet_list(&self, _: Vec<Snippet>) -> Option<Snippet> {
            self.selected.borrow().clone()
        }

        fn with_backup_list(&self, _: &[String]) -> Option<usize> {
            Some(0)
        }
    }

    struct MockClipboard {
        last_text: RefCell<Option<String>>,
        fail: bool,
    }

    impl ClipboardProvider for MockClipboard {
        fn set_text(&mut self, text: String) -> Result<(), String> {
            if self.fail {
                Err("Clipboard failed".to_string())
            } else {
                self.last_text.replace(Some(text));
                Ok(())
            }
        }
    }

    fn sample_snippet() -> Snippet {
        Snippet {
            name: "test".to_string(),
            description: "desc".to_string(),
            content: "echo hello".to_string(),
            executable: true,
            tags: vec!["dev".to_string()],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    #[test]
    fn test_copy_success() {
        let snippet = sample_snippet();

        let storage = MockStorage {
            snippets: vec![snippet.clone()],
            should_fail: false,
        };

        let selection_ui = MockSelectionUI {
            selected: RefCell::new(Some(snippet.clone())),
        };

        let mut clipboard = MockClipboard {
            last_text: RefCell::new(None),
            fail: false,
        };

        copy_command(
            &storage,
            &selection_ui,
            &mut clipboard,
            snippet.name.clone(),
        );

        assert_eq!(
            clipboard.last_text.borrow().as_deref(),
            Some(snippet.content.as_str())
        );
    }

    #[test]
    fn test_copy_storage_load_failure() {
        let storage = MockStorage {
            snippets: vec![],
            should_fail: true,
        };

        let selection_ui = MockSelectionUI {
            selected: RefCell::new(None),
        };

        let mut clipboard = MockClipboard {
            last_text: RefCell::new(None),
            fail: false,
        };

        copy_command(&storage, &selection_ui, &mut clipboard, "test".to_string());

        assert!(clipboard.last_text.borrow().is_none());
    }

    #[test]
    fn test_copy_snippet_not_found() {
        let snippet = sample_snippet();

        let storage = MockStorage {
            snippets: vec![snippet],
            should_fail: false,
        };

        let selection_ui = MockSelectionUI {
            selected: RefCell::new(None), // Simulate selection not found
        };

        let mut clipboard = MockClipboard {
            last_text: RefCell::new(None),
            fail: false,
        };

        copy_command(&storage, &selection_ui, &mut clipboard, "test".to_string());

        assert!(clipboard.last_text.borrow().is_none());
    }

    #[test]
    fn test_copy_clipboard_failure() {
        let snippet = sample_snippet();

        let storage = MockStorage {
            snippets: vec![snippet.clone()],
            should_fail: false,
        };

        let selection_ui = MockSelectionUI {
            selected: RefCell::new(Some(snippet)),
        };

        let mut clipboard = MockClipboard {
            last_text: RefCell::new(None),
            fail: true,
        };

        copy_command(&storage, &selection_ui, &mut clipboard, "test".to_string());

        assert!(clipboard.last_text.borrow().is_none());
    }
}
