use crate::commands::helper::get_snippet;

pub fn show_command(name: String) {
    let snippet = get_snippet(name).unwrap();

    println!("🔎 Snippet: {}", snippet.name);
    println!("📄 Description: {}", snippet.description);
    println!("📋 Content:\n{}", snippet.content);
    println!("🚀 Executable: {}", snippet.executable);
}
