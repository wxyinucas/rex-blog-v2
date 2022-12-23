use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FormArticle {
    pub title: String,
    pub content: String,
    pub category_id: i32,
    pub summary: String,
    pub state: i32,
    pub tags: String,
}

#[derive(Debug, Deserialize)]
pub struct FormCategory {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct FormTag {
    pub name: String,
}
