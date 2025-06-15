use crate::{models::Snippet, storage};
use std::io::{self, BufRead, Write};

pub fn save_command(name: String) -> () {
    let description = read_description_input();
    let executable = read_executable_input();
    let content = read_content_input();
    let entry = Snippet {
        name,
        description,
        content,
        executable,
    };

    storage::save_to_file(entry);
}

fn read_description_input() -> String {
    print!("📝 Enter description: ");
    io::stdout().flush().unwrap();

    let mut description = String::new();
    io::stdin().read_line(&mut description).unwrap();
    let description = description.trim().to_string();
    description
}

fn read_executable_input() -> bool {
    print!("🚀 Executable? (y/N): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let trimmed = input.trim().to_lowercase();

    matches!(trimmed.as_str(), "y" | "yes")
}

fn read_content_input() -> String {
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
