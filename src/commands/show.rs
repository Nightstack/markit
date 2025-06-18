use crate::{commands::helper::get_snippet, storage::Storage, ui::SelectionUI};

pub fn show_command(storage: &dyn Storage, selection_ui: &dyn SelectionUI, name: String) {
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

    println!("🔎 Snippet: {}", snippet.name);
    println!("📄 Description: {}", snippet.description);
    println!("🚀 Executable: {}", snippet.executable);
    println!("🕒 Created at: {}", snippet.created_at);
    println!("🕒 Updated at: {}", snippet.updated_at);
    println!("📋 Content:\n{}", snippet.content);
    println!("🏷️ Tags: {}", snippet.tags.join(", "));
}

#[cfg(test)]
mod tests {
    use super::*;
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

    struct MockSelectionUI {
        selection: RefCell<Option<Snippet>>,
    }

    impl SelectionUI for MockSelectionUI {
        fn with_snippet_list(&self, _: Vec<Snippet>) -> Option<Snippet> {
            self.selection.borrow().clone()
        }

        fn with_backup_list(&self, _: &[String]) -> Option<usize> {
            Some(0)
        }
    }

    #[test]
    fn test_show_command_success() {
        let snippet = Snippet {
            name: "test".to_string(),
            description: "desc".to_string(),
            content: "echo hello".to_string(),
            executable: true,
            tags: vec!["tag1".to_string()],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let storage = MockStorage {
            snippets: vec![snippet.clone()],
            should_fail: false,
        };

        let selection_ui = MockSelectionUI {
            selection: RefCell::new(Some(snippet)),
        };

        show_command(&storage, &selection_ui, "test".to_string());
    }

    #[test]
    fn test_show_command_load_failure() {
        let storage = MockStorage {
            snippets: vec![],
            should_fail: true,
        };

        let selection_ui = MockSelectionUI {
            selection: RefCell::new(None),
        };

        show_command(&storage, &selection_ui, "test".to_string());
    }

    #[test]
    fn test_show_command_not_found() {
        let snippet = Snippet {
            name: "test".to_string(),
            description: "desc".to_string(),
            content: "echo hello".to_string(),
            executable: true,
            tags: vec!["tag1".to_string()],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let storage = MockStorage {
            snippets: vec![snippet.clone()],
            should_fail: false,
        };

        let selection_ui = MockSelectionUI {
            selection: RefCell::new(None),
        };

        show_command(&storage, &selection_ui, "test".to_string());
    }
}
