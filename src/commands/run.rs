use crate::{
    command_runner::CommandRunner, commands::helper::get_snippet, storage::Storage, ui::SelectionUI,
};

pub fn run_command(
    storage: &dyn Storage,
    selection_ui: &dyn SelectionUI,
    runner: &dyn CommandRunner,
    name: String,
) {
    let store = match storage.load() {
        Ok(s) => s,
        Err(_) => {
            println!("📭 No snippets saved yet.");
            return;
        }
    };

    let snippet = match get_snippet(&store, selection_ui, name) {
        Some(s) => s,
        None => return,
    };

    if !snippet.executable {
        println!("⛔ Snippet '{}' not executable.", snippet.name);
        return;
    }

    println!("🚀 Running: {}", snippet.name);
    println!("📋 {}", snippet.content);

    match runner.run(&snippet.content) {
        Ok(code) if code.success() => println!("✅ Command ran successfully."),
        Ok(code) => println!("⚠️ Command exited with status: {}", code),
        Err(err) => println!("⛔ Failed to run command: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::Snippet,
        storage::{Storage, StorageError},
        ui::SelectionUI,
    };
    use chrono::Utc;
    use std::process::ExitStatus;

    struct MockStorage {
        snippet: Option<Snippet>,
        fail_load: bool,
    }

    impl Storage for MockStorage {
        fn load(&self) -> Result<crate::models::SnippetStore, StorageError> {
            if self.fail_load {
                Err(StorageError::Io(std::io::Error::other("Failed to load")))
            } else {
                Ok(crate::models::SnippetStore {
                    snippets: self.snippet.clone().into_iter().collect(),
                })
            }
        }

        fn save(&self, _: Snippet) -> Result<(), StorageError> {
            Ok(())
        }

        fn save_all(&self, _: &crate::models::SnippetStore) -> Result<(), StorageError> {
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
        fn with_snippet_list(&self, _snippets: Vec<Snippet>) -> Option<Snippet> {
            self.snippet.clone()
        }

        fn with_backup_list(&self, _backups: &[String]) -> Option<usize> {
            Some(0)
        }
    }

    struct MockCommandRunner {
        result: Result<ExitStatus, std::io::Error>,
    }

    impl CommandRunner for MockCommandRunner {
        fn run(&self, _command: &str) -> Result<ExitStatus, std::io::Error> {
            match &self.result {
                Ok(status) => Ok(*status),
                Err(e) => Err(std::io::Error::new(e.kind(), e.to_string())),
            }
        }
    }

    fn test_snippet(name: &str, executable: bool) -> Snippet {
        Snippet {
            name: name.to_string(),
            description: "desc".to_string(),
            content: "echo test".to_string(),
            executable,
            tags: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn fake_exit_status(success: bool) -> ExitStatus {
        if success {
            std::process::Command::new("true").status().unwrap()
        } else {
            std::process::Command::new("false").status().unwrap()
        }
    }

    #[test]
    fn test_run_success() {
        let snippet = test_snippet("test", true);

        let storage = MockStorage {
            snippet: Some(snippet.clone()),
            fail_load: false,
        };

        let ui = MockSelectionUI {
            snippet: Some(snippet),
        };

        let runner = MockCommandRunner {
            result: Ok(fake_exit_status(true)),
        };

        run_command(&storage, &ui, &runner, "test".to_string());
    }

    #[test]
    fn test_run_fails_to_execute() {
        let snippet = test_snippet("test", true);

        let storage = MockStorage {
            snippet: Some(snippet.clone()),
            fail_load: false,
        };

        let ui = MockSelectionUI {
            snippet: Some(snippet),
        };

        let runner = MockCommandRunner {
            result: Ok(fake_exit_status(false)),
        };

        run_command(&storage, &ui, &runner, "test".to_string());
    }

    #[test]
    fn test_run_command_error() {
        let snippet = test_snippet("test", true);

        let storage = MockStorage {
            snippet: Some(snippet.clone()),
            fail_load: false,
        };

        let ui = MockSelectionUI {
            snippet: Some(snippet),
        };

        let runner = MockCommandRunner {
            result: Err(std::io::Error::other("Mock error")),
        };

        run_command(&storage, &ui, &runner, "test".to_string());
    }

    #[test]
    fn test_run_not_executable() {
        let snippet = test_snippet("test", false);

        let storage = MockStorage {
            snippet: Some(snippet.clone()),
            fail_load: false,
        };

        let ui = MockSelectionUI {
            snippet: Some(snippet),
        };

        let runner = MockCommandRunner {
            result: Ok(fake_exit_status(true)),
        };

        run_command(&storage, &ui, &runner, "test".to_string());
    }

    #[test]
    fn test_run_load_failure() {
        let storage = MockStorage {
            snippet: None,
            fail_load: true,
        };

        let ui = MockSelectionUI { snippet: None };

        let runner = MockCommandRunner {
            result: Ok(fake_exit_status(true)),
        };

        run_command(&storage, &ui, &runner, "test".to_string());
    }

    #[test]
    fn test_run_not_found() {
        let storage = MockStorage {
            snippet: None,
            fail_load: false,
        };

        let ui = MockSelectionUI { snippet: None };

        let runner = MockCommandRunner {
            result: Ok(fake_exit_status(true)),
        };

        run_command(&storage, &ui, &runner, "test".to_string());
    }
}
