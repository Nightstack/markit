use chrono::Utc;

use crate::{input::SaveInput, models::Snippet, storage::Storage};

pub fn save_command(storage: &dyn Storage, input: &dyn SaveInput, name: String) -> () {
    if let Ok(store) = storage.load() {
        if store
            .snippets
            .iter()
            .any(|s| s.name.eq_ignore_ascii_case(&name))
        {
            eprintln!("⛔ A snippet with the name '{}' already exists.", name);
            return;
        }
    }

    let now = Utc::now();
    let entry = Snippet {
        name,
        description: input.get_description(),
        content: input.get_content(),
        executable: input.get_executable(),
        tags: input.get_tags(),
        created_at: now,
        updated_at: now,
    };

    match storage.save(entry) {
        Ok(_) => println!("✅ Snippet saved successfully."),
        Err(e) => eprintln!("⛔ Failed to save snippet: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::io;

    use crate::models::{Snippet, SnippetStore};
    use crate::storage::{Storage, StorageError};

    // Mock SaveInput
    struct MockInput;

    impl SaveInput for MockInput {
        fn get_description(&self) -> String {
            "Test description".to_string()
        }

        fn get_executable(&self) -> bool {
            true
        }

        fn get_content(&self) -> String {
            "echo 'Hello World'".to_string()
        }

        fn get_tags(&self) -> Vec<String> {
            vec!["test".to_string()]
        }
    }

    struct MockInputNoTags;

    impl SaveInput for MockInputNoTags {
        fn get_description(&self) -> String {
            "No tag snippet".to_string()
        }

        fn get_executable(&self) -> bool {
            false
        }

        fn get_content(&self) -> String {
            "echo 'No tags'".to_string()
        }

        fn get_tags(&self) -> Vec<String> {
            vec![]
        }
    }

    // Mock storage
    struct MockStorage {
        saved_snippets: RefCell<Vec<Snippet>>,
        fail_save: bool,
    }

    impl MockStorage {
        fn new() -> Self {
            Self {
                saved_snippets: RefCell::new(vec![]),
                fail_save: false,
            }
        }

        fn with_existing(snippet: Snippet) -> Self {
            Self {
                saved_snippets: RefCell::new(vec![snippet]),
                fail_save: false,
            }
        }

        fn with_failure() -> Self {
            Self {
                saved_snippets: RefCell::new(vec![]),
                fail_save: true,
            }
        }
    }

    impl Storage for MockStorage {
        fn load(&self) -> Result<SnippetStore, StorageError> {
            Ok(SnippetStore {
                snippets: self.saved_snippets.borrow().clone(),
            })
        }

        fn save(&self, snippet: Snippet) -> Result<(), StorageError> {
            if self.fail_save {
                return Err(StorageError::Io(io::Error::new(
                    io::ErrorKind::Other,
                    "Save failed",
                )));
            }
            self.saved_snippets.borrow_mut().push(snippet);
            Ok(())
        }

        fn save_all(&self, _store: &SnippetStore) -> Result<(), StorageError> {
            Ok(())
        }

        fn get_backups(&self) -> Result<Vec<std::path::PathBuf>, StorageError> {
            Ok(vec![])
        }

        fn restore_backup(&self, _path: &std::path::PathBuf) -> Result<(), StorageError> {
            Ok(())
        }
    }

    #[test]
    fn test_save_command_saves_snippet() {
        let storage = MockStorage::new();
        let input = MockInput;
        let name = "test_snippet".to_string();

        save_command(&storage, &input, name.clone());

        let snippets = storage.saved_snippets.borrow();
        assert_eq!(snippets.len(), 1);

        let saved = &snippets[0];
        assert_eq!(saved.name, name);
        assert_eq!(saved.description, "Test description");
        assert_eq!(saved.content, "echo 'Hello World'");
        assert!(saved.executable);
        assert_eq!(saved.tags, vec!["test"]);
    }

    #[test]
    fn test_save_command_rejects_duplicate() {
        let existing_snippet = Snippet {
            name: "duplicate".to_string(),
            description: "Existing".to_string(),
            content: "echo test".to_string(),
            executable: false,
            tags: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let storage = MockStorage::with_existing(existing_snippet);
        let input = MockInput;

        save_command(&storage, &input, "duplicate".to_string());

        // Should not save another
        let snippets = storage.saved_snippets.borrow();
        assert_eq!(snippets.len(), 1);
        assert_eq!(snippets[0].name, "duplicate");
    }

    #[test]
    fn test_save_command_handles_storage_error() {
        let storage = MockStorage::with_failure();
        let input = MockInput;

        // Should not panic
        save_command(&storage, &input, "fail_test".to_string());

        let snippets = storage.saved_snippets.borrow();
        assert!(snippets.is_empty());
    }

    #[test]
    fn test_save_command_with_no_tags() {
        let storage = MockStorage::new();
        let input = MockInputNoTags;

        save_command(&storage, &input, "no_tags".to_string());

        let snippets = storage.saved_snippets.borrow();
        assert_eq!(snippets.len(), 1);
        assert!(snippets[0].tags.is_empty());
    }
}
