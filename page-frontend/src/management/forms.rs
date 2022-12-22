#![allow(clippy::all, unused_imports, dead_code)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct FormArticle {
    pub title: String,
    pub content: String,
    pub category_id: i32,
    pub summary: String,
    pub state: i32,
    pub tags: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct FormCategory {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct FormTag {
    pub name: String,
}
