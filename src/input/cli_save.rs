use crate::input::SaveInput;
use std::io::{self, BufRead, Write};

pub struct CliSaveInput;

impl SaveInput for CliSaveInput {
    fn get_description(&self) -> String {
        print!("📝 Enter description: ");
        io::stdout().flush().unwrap();

        let mut description = String::new();
        io::stdin().read_line(&mut description).unwrap();
        description.trim().to_string()
    }

    fn get_executable(&self) -> bool {
        print!("🚀 Executable? (y/N): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim().to_lowercase();

        matches!(trimmed.as_str(), "y" | "yes")
    }

    fn get_content(&self) -> String {
        println!("💡 Paste your command below.");
        println!("👉 End with either:");
        println!("   - Ctrl+D (Unix/macOS) or Ctrl+Z then Enter (Windows)");
        println!("   - Or type 'EOF' or '---' on a new line to finish:");

        let mut content = String::new();

        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(l) if l.trim() == "EOF" || l.trim() == "---" => break,
                Ok(l) => {
                    content.push_str(&l);
                    content.push('\n');
                }
                Err(err) => {
                    eprintln!("⛔ Error reading input: {}", err);
                    break;
                }
            }
        }

        content
    }

    fn get_tags(&self) -> Vec<String> {
        print!("🏷️  Enter tags (comma-separated, optional): ");
        io::stdout().flush().unwrap();

        let mut tags_input = String::new();
        io::stdin().read_line(&mut tags_input).unwrap();

        tags_input
            .split(',')
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty())
            .collect()
    }
}
