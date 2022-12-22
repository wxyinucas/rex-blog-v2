use chrono::{DateTime, Datelike, Duration, Local, TimeZone};
use prost_types::Timestamp;
use serde::ser::SerializeStruct;
use serde::Serializer;
use sqlx::postgres::PgRow;
use sqlx::{Error, FromRow, Row};

use crate::{get_summary, Article, ArticleState, QueryArticle, QueryCategory, QueryTag};

/* =================================================================


Query to Sql String


================================================================== */
pub trait ToSql {
    fn to_sql(self) -> String;
}

impl ToSql for QueryArticle {
    fn to_sql(self) -> String {
        let ids = if self.ids.is_empty() {
            "True".to_string()
        } else {
            format!(
                "id IN ({})",
                self.ids
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            )
        };

        let title = if self.title.is_empty() {
            "True".to_string()
        } else {
            format!("title like '%{}%'", self.title)
        };

        let state = if self.state == 0 {
            "True".to_string()
        } else {
            format!("state = {}", self.state)
        };

        let created_year = if self.created_year.is_none() {
            "True".to_string()
        } else {
            let ts = self.created_year.as_ref().unwrap();
            let year = to_chrono(ts).year();
            let local_start = Local
                .from_local_datetime(
                    &chrono::NaiveDate::from_ymd_opt(year, 1, 1)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                )
                .unwrap();
            let local_end = local_start + Duration::days(365);

            format!(
                "created_at >= '{}' AND created_at < '{}'",
                local_start, local_end
            )
        };

        let category_id = if self.category_id == 0 {
            "True".to_string()
        } else {
            format!("category_id = {}", self.category_id)
        };

        format!(
            "{} AND {} AND {} AND {} AND {}",
            ids, title, state, created_year, category_id
        )
    }
}

impl ToSql for QueryCategory {
    fn to_sql(self) -> String {
        let ids = if self.ids.is_empty() {
            "True".to_string()
        } else {
            format!(
                "id IN ({})",
                self.ids
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            )
        };

        let name = if self.name.is_empty() {
            "True".to_string()
        } else {
            format!("name like '%{}%'", self.name)
        };

        format!("{} AND {}", ids, name)
    }
}

impl ToSql for QueryTag {
    fn to_sql(self) -> String {
        let ids = if self.ids.is_empty() {
            "True".to_string()
        } else {
            format!(
                "id IN ({})",
                self.ids
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            )
        };

        let name = if self.name.is_empty() {
            "True".to_string()
        } else {
            format!("name like '%{}%'", self.name)
        };

        format!("{} AND {}", ids, name)
    }
}
/* =================================================================


Update to Sql String


================================================================== */
impl ToSql for Article {
    fn to_sql(self) -> String {
        let update_at = format!("updated_at = '{}',", Local::now());

        let title = if self.title.is_empty() {
            "".to_string()
        } else {
            format!("title = '{}',", self.title)
        };

        let content = if self.content.is_empty() {
            "".to_string()
        } else {
            format!("content = '{}',", self.content)
        };

        let summary = if self.summary.is_empty() && self.content.is_empty() {
            "".to_string()
        } else if !self.summary.is_empty() {
            format!("summary = '{}',", self.summary)
        } else {
            format!("summary = '{}',", get_summary(&self.content))
        };

        let state = if self.state == ArticleState::All as i32 {
            "".to_string()
        } else {
            format!(
                "state = '{}',",
                ArticleState::try_from(self.state)
                    .unwrap()
                    .as_str_name()
                    .to_lowercase()
            )
        };

        let category_id = if self.category_id == 0 {
            "".to_string()
        } else {
            format!("category_id = {},", self.category_id)
        };

        format!(
            "{}{}{}{}{}{}",
            update_at, title, content, summary, state, category_id
        )
        .trim_end_matches(',')
        .to_string()
    }
}

/* =================================================================


util time transfer functions


================================================================== */
fn to_chrono(time: &Timestamp) -> chrono::DateTime<Local> {
    let ts = time.seconds;
    let nanos = time.nanos;
    let dt = chrono::NaiveDateTime::from_timestamp_opt(ts, nanos as _).unwrap();
    chrono::DateTime::<Local>::from_utc(dt, chrono::FixedOffset::east_opt(8 * 3600).unwrap())
}

pub fn to_timestamp(time: chrono::DateTime<Local>) -> Timestamp {
    let ts = time.timestamp();
    let nanos = time.timestamp_subsec_nanos();
    Timestamp {
        seconds: ts,
        nanos: nanos as _,
    }
}

/* =================================================================


sqlx transfer functions


================================================================== */
#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "article_state", rename_all = "lowercase")]
pub enum AS {
    All,
    Published,
    Hidden,
}

impl From<AS> for ArticleState {
    fn from(value: AS) -> Self {
        match value {
            AS::All => ArticleState::All,
            AS::Published => ArticleState::Published,
            AS::Hidden => ArticleState::Hidden,
        }
    }
}

impl TryFrom<i32> for ArticleState {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ArticleState::All),
            1 => Ok(ArticleState::Published),
            2 => Ok(ArticleState::Hidden),
            _ => Err("No such state".to_string()),
        }
    }
}

impl TryFrom<i32> for AS {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(AS::All),
            1 => Ok(AS::Published),
            2 => Ok(AS::Hidden),
            other => Err(format!("AS: {} not implement", other)),
        }
    }
}

impl FromRow<'_, PgRow> for Article {
    fn from_row(row: &'_ PgRow) -> Result<Self, Error> {
        let created_at = row.try_get::<DateTime<Local>, _>("created_at")?;
        let updated_at = row.try_get::<DateTime<Local>, _>("updated_at")?;
        let state = ArticleState::from(row.try_get::<AS, _>("state")?) as i32;
        let tag_ids = row.try_get::<Vec<i32>, _>("tag_ids")?;

        Ok(Article {
            id: row.try_get("id")?,
            title: row.try_get("title")?,
            content: row.try_get("content")?,
            summary: row.try_get("summary")?,
            state,
            created_at: Some(to_timestamp(created_at)),
            updated_at: Some(to_timestamp(updated_at)),
            category_id: row.try_get("category_id")?,
            tags_id: tag_ids,
        })
    }
}

/* =================================================================


Serialize


================================================================== */

impl serde::Serialize for Article {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Article", 7)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("content", &self.content)?;
        state.serialize_field("category_id", &self.category_id)?;
        state.serialize_field("summary", &self.summary)?;
        state.serialize_field("state", &self.state)?;

        let create_at = to_chrono(self.created_at.as_ref().unwrap());
        let create_at = create_at.format("%Y-%m-%d %H:%M").to_string();
        state.serialize_field("created_at", &create_at)?;
        let update_at = to_chrono(self.updated_at.as_ref().unwrap());
        let update_at = update_at.format("%Y-%m-%d %H:%M").to_string();
        state.serialize_field("updated_at", &update_at)?;
        state.serialize_field("tags_id", &self.tags_id)?;
        state.end()
    }
}

/* =================================================================


tests


================================================================== */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_chrono_should_work() {
        let now = Local::now();
        let time = to_timestamp(now);
        let new_time = to_chrono(&time);
        println!("{:?}", new_time);
    }

    #[test]
    fn article_serialize_should_work() {
        let article = Article {
            id: 1,
            title: "title".into(),
            content: "content".into(),
            category_id: 1,
            summary: "summary".into(),
            state: 1,
            created_at: Some(Timestamp {
                seconds: 1,
                nanos: 1,
            }),
            updated_at: Some(Timestamp {
                seconds: 1,
                nanos: 1,
            }),
            tags_id: vec![1],
        };

        let json = serde_json::to_string_pretty(&article).unwrap();
        let res = r#"{
  "id": 1,
  "title": "title",
  "content": "content",
  "category_id": 1,
  "summary": "summary",
  "state": 1,
  "created_at": "1970-01-01 08:00",
  "updated_at": "1970-01-01 08:00",
  "tags_id": [
    1
  ]
}"#;
        assert_eq!(json, res);
        // println!("{}", json);
    }
}
