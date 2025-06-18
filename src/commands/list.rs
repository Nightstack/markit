use crate::{
    models::Snippet,
    storage::{
        Storage,
        filter::{Filter, apply_filter},
    },
    ui::TableUI,
};

pub fn list_command(storage: &dyn Storage, table_ui: &mut dyn TableUI, tag: Option<String>) {
    let store = match storage.load() {
        Ok(s) => s,
        Err(_) => {
            println!("📭 No snippets saved yet.");
            return;
        }
    };

    let snippets: Vec<Snippet> = match tag.as_deref() {
        Some(tag) => apply_filter(&store, Filter::Tag(tag.to_string())),
        None => apply_filter(&store, Filter::All),
    };

    if snippets.is_empty() {
        if let Some(tag) = tag {
            println!("📭 No snippets found for tag: {}.", tag);
        } else {
            println!("📭 No snippets saved yet.");
        }
    } else {
        let table = table_ui.with_snippet_list(snippets);
        println!("{table}");
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::{
        commands::list::list_command,
        models::{Snippet, SnippetStore},
        storage::{Storage, StorageError},
        ui::TableUI,
    };

    struct MockStorage {
        store: SnippetStore,
    }

    impl Storage for MockStorage {
        fn load(&self) -> Result<SnippetStore, StorageError> {
            Ok(self.store.clone())
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

    struct MockTableUI {
        printed_table: Rc<RefCell<bool>>,
    }

    impl TableUI for MockTableUI {
        fn with_snippet_list(&mut self, _: Vec<Snippet>) -> comfy_table::Table {
            *self.printed_table.borrow_mut() = true;
            comfy_table::Table::new()
        }
    }

    #[test]
    fn test_list_command_no_snippets_in_store() {
        let storage = MockStorage {
            store: SnippetStore { snippets: vec![] },
        };

        let mut table_ui = MockTableUI {
            printed_table: Rc::new(RefCell::new(false)),
        };

        list_command(&storage, &mut table_ui, None);
        assert!(!*table_ui.printed_table.borrow());
    }

    #[test]
    fn test_list_command_snippets_with_tag_no_match() {
        let storage = MockStorage {
            store: SnippetStore {
                snippets: vec![Snippet {
                    name: "test".to_string(),
                    description: "test desc".to_string(),
                    content: "ls".to_string(),
                    executable: true,
                    tags: vec!["dev".to_string()],
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                }],
            },
        };

        let mut table_ui = MockTableUI {
            printed_table: Rc::new(RefCell::new(false)),
        };

        list_command(&storage, &mut table_ui, Some("nonexistent".to_string()));
        assert!(!*table_ui.printed_table.borrow());
    }

    #[test]
    fn test_list_command_snippets_with_results() {
        let storage = MockStorage {
            store: SnippetStore {
                snippets: vec![Snippet {
                    name: "test".to_string(),
                    description: "test desc".to_string(),
                    content: "ls".to_string(),
                    executable: true,
                    tags: vec!["dev".to_string()],
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                }],
            },
        };

        let mut table_ui = MockTableUI {
            printed_table: Rc::new(RefCell::new(false)),
        };

        list_command(&storage, &mut table_ui, None);
        assert!(*table_ui.printed_table.borrow());
    }
}
