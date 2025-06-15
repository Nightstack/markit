use crate::{models::Snippet, storage};
use std::io::{self, BufRead, Write};

pub fn save_command(name: String) -> () {
    let description = read_description_input();
    let content = read_content_input();
    let entry = Snippet {
        name,
        description,
        content,
        executable: true,
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

fn read_content_input() -> String {
    println!("💡 type/paste your command below (end with 'EOF' on a new line):");
    let stdin = io::stdin();
    let mut content = String::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.trim() == "EOF" {
            break;
        }

        content.push_str(&line);
        content.push('\n');
    }
    content
}
