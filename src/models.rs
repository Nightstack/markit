use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Snippet {
    pub name: String,
    pub description: String,
    pub content: String,
    pub executable: bool,
}

#[derive(Serialize, Deserialize, Default)]
pub struct SnippetStore {
    pub snippets: Vec<Snippet>,
}
