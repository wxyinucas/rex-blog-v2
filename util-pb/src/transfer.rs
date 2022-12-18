use chrono::{DateTime, Datelike, Duration, Local, TimeZone};
use prost_types::Timestamp;
use sqlx::postgres::PgRow;
use sqlx::{Error, FromRow, Row};

use crate::{Article, ArticleState, QueryArticle};

/* =================================================================


Query to Sql String


================================================================== */
#[tonic::async_trait]
pub trait ToSql {
    fn to_sql(self) -> String;
}

#[tonic::async_trait]
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