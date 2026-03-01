use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    pub id: String,
    pub name: String,
    pub title: String,
    pub author: String,
    pub betaka: String,
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub sub_categories: Vec<Category>,
    pub books: Vec<Book>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    pub id: String,
    pub title: String,
    pub level: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub id: String,
    pub nass: String,
    pub part: String,
    pub page: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Library {
    pub categories: Vec<Category>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub book_id: String,
    pub book_title: String,
    pub author: String,
    pub part: String,
    pub page: String,
    pub snippet: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotation {
    pub id: String,
    pub book_id: String,
    pub page_id: String,
    pub text: String,
    pub color: String,
    pub note: Option<String>,
    pub timestamp: u64,
}
