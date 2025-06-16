use crate::commands::helper::get_snippet;

pub fn show_command(name: String) {
    let snippet = get_snippet(name).unwrap();

    println!("🔎 Snippet: {}", snippet.name);
    println!("📄 Description: {}", snippet.description);
    println!("🚀 Executable: {}", snippet.executable);
    println!("🕒 Created at: {}", snippet.created_at);
    println!("🕒 Updated at: {}", snippet.updated_at);
    println!("📋 Content:\n{}", snippet.content);
    println!("🏷️ Tags: {}", snippet.tags.join(", "));
}
