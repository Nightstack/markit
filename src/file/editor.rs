use crate::{file::EditorLauncher, models::PartialSnippet};
use std::io::Write;

pub struct Editor;

impl EditorLauncher for Editor {
    fn open_editor(&self, snippet: &PartialSnippet) -> Result<PartialSnippet, String> {
        let mut tmpfile = tempfile::NamedTempFile::new()
            .map_err(|e| format!("Could not create temp file: {}", e))?;

        let yaml = serde_yaml::to_string(snippet)
            .map_err(|e| format!("Could not serialize snippet: {}", e))?;

        tmpfile
            .write_all(yaml.as_bytes())
            .map_err(|e| format!("Could not write to temp file: {}", e))?;

        let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());

        let status = std::process::Command::new(editor)
            .arg(tmpfile.path())
            .status()
            .map_err(|e| format!("Failed to launch editor: {}", e))?;

        if !status.success() {
            return Err("Editor exited with an error.".to_string());
        }

        let contents = std::fs::read_to_string(tmpfile.path())
            .map_err(|e| format!("Could not read edited file: {}", e))?;

        let edited: PartialSnippet =
            serde_yaml::from_str(&contents).map_err(|_| "Invalid YAML.".to_string())?;

        Ok(edited)
    }
}
