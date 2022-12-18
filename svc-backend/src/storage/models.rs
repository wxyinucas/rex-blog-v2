#![allow(clippy::all, dead_code, unused_imports)]
use chrono::{DateTime, Local};
use sqlx::postgres::PgRow;
use sqlx::{Error, FromRow};

use util_pb::{to_timestamp, Article};

use crate::storage::traits::ID;

#[derive(sqlx::FromRow)]
pub(crate) struct DBArticle {
    pub id: ID,
    pub title: String,
    pub content: String,
    pub summary: String,
    pub category_id: ID,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub tag_ids: Vec<ID>,
}

// impl From<DBArticle> for Article {
//     fn from(article: DBArticle) -> Self {
//         Article {
//             id: article.id,
//             title: article.title,
//             content: article.content,
//             summary: article.summary,
//             state: article.category_id,
//             created_at: Some(to_timestamp(article.created_at)),
//             updated_at: Some(to_timestamp(article.updated_at)),
//             cat_id: article.category_id,
//
//             tags_id: article.tag_ids,
//         }
//     }
// }
