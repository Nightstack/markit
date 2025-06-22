use crate::{
    commands::helper::{get_snippet, redact_snippet},
    file::EditorLauncher,
    models::{PartialSnippet, Snippet},
    storage::Storage,
    ui::SelectionUI,
};

pub fn edit_command(
    storage: &dyn Storage,
    selection_ui: &dyn SelectionUI,
    editor: &dyn EditorLauncher,
    name: String,
) {
    let mut store = match storage.load() {
        Ok(s) => s,
        Err(_) => {
            println!("📭 No snippets saved yet.");
            return;
        }
    };

    let mut original = match get_snippet(&store, selection_ui, name) {
        Some(s) => s,
        None => {
            return;
        }
    };

    let editable = redact_snippet(&original);

    let edited = match editor.open_editor(&editable) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("⛔ {}", e);
            return;
        }
    };

    if store
        .snippets
        .iter()
        .any(|s| s.name.eq_ignore_ascii_case(&edited.name) && s.name != original.name)
    {
        eprintln!(
            "⛔ Another snippet with the name '{}' already exists.",
            edited.name
        );
        return;
    }

    store.snippets.retain(|s| s.name != original.name);
    apply_edits(&mut original, edited);
    store.snippets.push(original.clone());

    if let Err(err) = storage.save_all(&store) {
        eprintln!("⛔ Failed to update snippet: {:?}", err);
    } else {
        println!("✏️ Snippet '{}' updated.", original.name);
    }
}

fn apply_edits(original: &mut Snippet, edited: PartialSnippet) {
    original.name = edited.name;
    original.description = edited.description;
    original.content = edited.content;
    original.executable = edited.executable;
    original.updated_at = chrono::Utc::now();
    original.tags = edited.tags;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file::EditorLauncher;
    use crate::models::{PartialSnippet, Snippet, SnippetStore};
    use crate::storage::{Storage, StorageError};
    use crate::ui::SelectionUI;
    use std::cell::RefCell;

    struct MockStorage {
        store: RefCell<SnippetStore>,
        fail_save: bool,
    }

    impl Storage for MockStorage {
        fn load(&self) -> Result<SnippetStore, StorageError> {
            Ok(self.store.borrow().clone())
        }

        fn save(&self, _: Snippet) -> Result<(), StorageError> {
            Ok(())
        }

        fn save_all(&self, store: &SnippetStore) -> Result<(), StorageError> {
            if self.fail_save {
                return Err(StorageError::Io(std::io::Error::other("save failed")));
            }
            self.store.replace(store.clone());
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
        snippet: RefCell<Option<Snippet>>,
    }

    impl SelectionUI for MockSelectionUI {
        fn with_snippet_list(&self, _: Vec<Snippet>) -> Option<Snippet> {
            self.snippet.borrow().clone()
        }

        fn with_backup_list(&self, _: &[String]) -> Option<usize> {
            Some(0)
        }
    }

    struct MockEditorLauncher {
        result: Result<PartialSnippet, String>,
    }

    impl EditorLauncher for MockEditorLauncher {
        fn open_editor(&self, _: &PartialSnippet) -> Result<PartialSnippet, String> {
            self.result.clone()
        }
    }

    fn make_test_snippet() -> Snippet {
        Snippet {
            name: "test".into(),
            description: "desc".into(),
            content: "echo hello".into(),
            executable: true,
            tags: vec!["tag1".into()],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    fn make_partial_snippet() -> PartialSnippet {
        PartialSnippet {
            name: "test-edited".into(),
            description: "new desc".into(),
            content: "echo world".into(),
            executable: false,
            tags: vec!["tag2".into()],
        }
    }

    #[test]
    fn test_edit_command_success() {
        let original = make_test_snippet();
        let store = SnippetStore {
            snippets: vec![original.clone()],
        };

        let storage = MockStorage {
            store: RefCell::new(store),
            fail_save: false,
        };

        let selection_ui = MockSelectionUI {
            snippet: RefCell::new(Some(original.clone())),
        };

        let editor = MockEditorLauncher {
            result: Ok(make_partial_snippet()),
        };

        edit_command(&storage, &selection_ui, &editor, original.name.clone());
        let updated = &storage.store.borrow().snippets[0];
        assert_eq!(updated.name, "test-edited");
        assert_eq!(updated.description, "new desc");
        assert_eq!(updated.content, "echo world");
        assert!(!updated.executable);
        assert_eq!(updated.tags, vec!["tag2"]);
    }

    #[test]
    fn test_edit_command_editor_fail() {
        let original = make_test_snippet();
        let store = SnippetStore {
            snippets: vec![original.clone()],
        };

        let storage = MockStorage {
            store: RefCell::new(store),
            fail_save: false,
        };

        let selection_ui = MockSelectionUI {
            snippet: RefCell::new(Some(original.clone())),
        };

        let editor = MockEditorLauncher {
            result: Err("Editor error".into()),
        };

        edit_command(&storage, &selection_ui, &editor, original.name.clone());

        // Should not have changed
        let unchanged = &storage.store.borrow().snippets[0];
        assert_eq!(unchanged.name, original.name);
    }

    #[test]
    fn test_edit_command_duplicate_name() {
        let snippet1 = make_test_snippet();
        let mut snippet2 = snippet1.clone();
        snippet2.name = "other".into();

        let store = SnippetStore {
            snippets: vec![snippet1.clone(), snippet2.clone()],
        };

        let storage = MockStorage {
            store: RefCell::new(store),
            fail_save: false,
        };

        let selection_ui = MockSelectionUI {
            snippet: RefCell::new(Some(snippet2.clone())),
        };

        let mut partial = make_partial_snippet();
        partial.name = snippet1.name.clone(); // Cause conflict

        let editor = MockEditorLauncher {
            result: Ok(partial),
        };

        edit_command(&storage, &selection_ui, &editor, snippet2.name.clone());

        // Should still have both original names
        let names: Vec<_> = storage
            .store
            .borrow()
            .snippets
            .iter()
            .map(|s| s.name.clone())
            .collect();
        assert!(names.contains(&"test".to_string()));
        assert!(names.contains(&"other".to_string()));
    }
}
