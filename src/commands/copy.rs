use arboard::Clipboard;

use crate::commands::helper::get_snippet;

pub fn copy_command(name: String) -> () {
    let snippet = get_snippet(name).unwrap();

    let mut clipboard = Clipboard::new().expect("Failed to access clipboard");
    clipboard
        .set_text(snippet.content.clone())
        .expect("Failed to copy to clipboard");
    println!("📋 Snippet '{}' copied to clipboard", snippet.name);
}
