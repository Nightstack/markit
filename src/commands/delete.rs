use crate::{
    commands::helper::get_snippet,
    storage::Storage,
    ui::{ConfirmPrompt, SelectionUI},
};

pub fn delete_command(
    storage: &dyn Storage,
    selection_ui: &dyn SelectionUI,
    confirm: &dyn ConfirmPrompt,
    name: String,
    force: bool,
) {
    let mut store = match storage.load() {
        Ok(s) => s,
        Err(_) => {
            println!("📭 No snippets saved yet.");
            return;
        }
    };

    let delete_snippet = match get_snippet(&store, selection_ui, name) {
        Some(s) => s,
        None => {
            return;
        }
    };

    if !force {
        let prompt = format!(
            "❗ Are you sure you want to delete '{}'? This cannot be undone.",
            delete_snippet.name
        );
        if !confirm.confirm(&prompt) {
            println!("🚫 Deletion cancelled.");
            return;
        }
    }

    store.snippets.retain(|s| s.name != delete_snippet.name);

    if let Err(err) = storage.save_all(&store) {
        eprintln!("⛔ Failed to update snippets file: {:?}", err);
    } else {
        println!("🗑️ Snippet '{}' deleted.", delete_snippet.name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Snippet, SnippetStore};
    use crate::storage::{Storage, StorageError};
    use crate::ui::{ConfirmPrompt, SelectionUI};
    use std::cell::RefCell;

    struct MockStorage {
        pub snippets: RefCell<Vec<Snippet>>,
        pub should_fail_load: bool,
        pub should_fail_save: bool,
    }

    impl Storage for MockStorage {
        fn load(&self) -> Result<SnippetStore, StorageError> {
            if self.should_fail_load {
                return Err(StorageError::Io(std::io::Error::other("Load failed")));
            }

            Ok(SnippetStore {
                snippets: self.snippets.borrow().clone(),
            })
        }

        fn save(&self, _: Snippet) -> Result<(), StorageError> {
            Ok(())
        }

        fn save_all(&self, store: &SnippetStore) -> Result<(), StorageError> {
            if self.should_fail_save {
                return Err(StorageError::Io(std::io::Error::other("Save failed")));
            }

            self.snippets.replace(store.snippets.clone());
            Ok(())
        }

        fn get_backups(&self) -> Result<Vec<std::path::PathBuf>, StorageError> {
            Ok(vec![])
        }

        fn restore_backup(&self, _: &std::path::Path) -> Result<(), StorageError> {
            Ok(())
        }
    }

    struct MockSelectionUI {
        snippet: Option<Snippet>,
    }

    impl SelectionUI for MockSelectionUI {
        fn with_snippet_list(&self, _: Vec<Snippet>) -> Option<Snippet> {
            self.snippet.clone()
        }

        fn with_backup_list(&self, _: &[String]) -> Option<usize> {
            Some(0)
        }
    }

    struct MockConfirmPrompt {
        confirm_result: bool,
    }

    impl ConfirmPrompt for MockConfirmPrompt {
        fn confirm(&self, _: &str) -> bool {
            self.confirm_result
        }
    }

    fn sample_snippet(name: &str) -> Snippet {
        Snippet {
            name: name.to_string(),
            description: "desc".into(),
            content: "echo test".into(),
            executable: true,
            tags: vec!["tag".into()],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    #[test]
    fn test_delete_with_force() {
        let snippet = sample_snippet("test");
        let storage = MockStorage {
            snippets: RefCell::new(vec![snippet.clone()]),
            should_fail_load: false,
            should_fail_save: false,
        };
        let selection_ui = MockSelectionUI {
            snippet: Some(snippet),
        };
        let confirm = MockConfirmPrompt {
            confirm_result: false,
        }; // Should be skipped

        delete_command(&storage, &selection_ui, &confirm, "test".to_string(), true);

        assert!(storage.snippets.borrow().is_empty());
    }

    #[test]
    fn test_delete_with_confirmation_yes() {
        let snippet = sample_snippet("test");
        let storage = MockStorage {
            snippets: RefCell::new(vec![snippet.clone()]),
            should_fail_load: false,
            should_fail_save: false,
        };
        let selection_ui = MockSelectionUI {
            snippet: Some(snippet),
        };
        let confirm = MockConfirmPrompt {
            confirm_result: true,
        };

        delete_command(&storage, &selection_ui, &confirm, "test".to_string(), false);

        assert!(storage.snippets.borrow().is_empty());
    }

    #[test]
    fn test_delete_with_confirmation_no() {
        let snippet = sample_snippet("test");
        let storage = MockStorage {
            snippets: RefCell::new(vec![snippet.clone()]),
            should_fail_load: false,
            should_fail_save: false,
        };
        let selection_ui = MockSelectionUI {
            snippet: Some(snippet.clone()),
        };
        let confirm = MockConfirmPrompt {
            confirm_result: false,
        };

        delete_command(&storage, &selection_ui, &confirm, "test".to_string(), false);

        assert_eq!(storage.snippets.borrow().len(), 1);
    }

    #[test]
    fn test_delete_snippet_not_found() {
        let storage = MockStorage {
            snippets: RefCell::new(vec![]),
            should_fail_load: false,
            should_fail_save: false,
        };
        let selection_ui = MockSelectionUI { snippet: None };
        let confirm = MockConfirmPrompt {
            confirm_result: true,
        };

        delete_command(
            &storage,
            &selection_ui,
            &confirm,
            "missing".to_string(),
            true,
        );

        assert!(storage.snippets.borrow().is_empty());
    }

    #[test]
    fn test_delete_load_failure() {
        let storage = MockStorage {
            snippets: RefCell::new(vec![]),
            should_fail_load: true,
            should_fail_save: false,
        };
        let selection_ui = MockSelectionUI {
            snippet: Some(sample_snippet("irrelevant")),
        };
        let confirm = MockConfirmPrompt {
            confirm_result: true,
        };

        delete_command(&storage, &selection_ui, &confirm, "test".to_string(), true);
    }

    #[test]
    fn test_delete_save_failure() {
        let snippet = sample_snippet("test");
        let storage = MockStorage {
            snippets: RefCell::new(vec![snippet.clone()]),
            should_fail_load: false,
            should_fail_save: true,
        };
        let selection_ui = MockSelectionUI {
            snippet: Some(snippet),
        };
        let confirm = MockConfirmPrompt {
            confirm_result: true,
        };

        delete_command(&storage, &selection_ui, &confirm, "test".to_string(), false);
    }
}
