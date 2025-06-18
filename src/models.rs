use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Snippet {
    pub name: String,
    pub description: String,
    pub content: String,
    pub executable: bool,
    pub tags: Vec<String>,
    #[serde(default = "default_now")]
    pub created_at: DateTime<Utc>,
    #[serde(default = "default_now")]
    pub updated_at: DateTime<Utc>,
}

fn default_now() -> DateTime<Utc> {
    Utc::now()
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct SnippetStore {
    pub snippets: Vec<Snippet>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PartialSnippet {
    pub name: String,
    pub description: String,
    pub content: String,
    pub executable: bool,
    pub tags: Vec<String>,
}
