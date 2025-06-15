use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Snippet {
    pub name: String,
    pub description: String,
    pub content: String,
    pub executable: bool,
}
