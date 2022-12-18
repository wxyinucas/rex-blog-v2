#![allow(clippy::all, dead_code, unused_imports, unused_variables)]

use std::ops::Deref;

use sqlx::Row;

use util_pb::transfer::AS;
use util_pb::{
    to_timestamp, transfer::ToSql, Article, ArticleState, Category, QueryArticle, QueryCategory,
    QueryTag, Tag,
};

use crate::error::Result;
use crate::storage::models::DBArticle;
use crate::storage::traits::{BlogDB, ID};
use crate::storage::DBPool;

#[tonic::async_trait]
impl BlogDB for DBPool {
    async fn query_articles(&self, mut req: QueryArticle) -> Result<Vec<Article>> {
        let mut article_ids = if req.tags_id.is_empty() {
            vec![]
        } else {
            let sql = "SELECT article_id FROM blog.article_tag WHERE tag_id = ANY($1) group by article_id having count(article_id) >= $2;"; // 逻辑：并
            let rows = sqlx::query(sql)
                .bind(&req.tags_id)
                .bind(req.tags_id.len() as i32)
                .fetch_all(self.deref())
                .await?;
            rows.into_iter().map(|row| row.get(0)).collect::<Vec<i32>>()
        };

        req.ids.append(&mut article_ids);
        let condition = req.to_sql();
        let sql = format!("SELECT * FROM blog.articles  WHERE {}", condition);
        let sql = format!("SELECT * FROM ({}) AS first LEFT OUTER JOIN blog.article2tag AS second ON first.id = second.article_id;", sql);

        // let articles = sqlx::query_as::<_, DBArticle>(&sql)
        //     .fetch_all(self.deref())
        //     .await?;
        // let res = articles
        //     .into_iter()
        //     .map(|article| {
        //         let article = Article::from(article); // todo 增加tags
        //         article
        //     })
        //     .collect::<Vec<_>>();

        let res = sqlx::query_as::<_, Article>(&sql)
            .fetch_all(self.deref())
            .await?;
        Ok(res)
    }

    async fn add_article(&self, article: Article) -> Result<ID> {
        let sql = "INSERT INTO blog.articles (title, content, summary, state, category_id) VALUES ($1, $2, $3, $4, $5) RETURNING id;";
        let state = AS::try_from(article.state).unwrap();

        let id = sqlx::query(sql)
            .bind(&article.title)
            .bind(&article.content)
            .bind(article.summary)
            .bind(state)
            .bind(article.category_id)
            .fetch_one(self.deref())
            .await?
            .get(0);

        let sql = "INSERT INTO blog.article_tag (article_id, tag_id) VALUES ($1, $2);";
        for tag_id in article.tags_id {
            sqlx::query(sql)
                .bind(id)
                .bind(tag_id)
                .execute(self.deref())
                .await?;
        }
        Ok(id)
    }

    async fn edit_article(&self, article: Article) -> Result<ID> {
        todo!()
    }

    async fn delete_article(&self, id: ID) -> Result<()> {
        todo!()
    }

    async fn query_categories(&self, req: QueryCategory) -> Result<Vec<Category>> {
        todo!()
    }

    async fn add_category(&self, category: Category) -> Result<ID> {
        todo!()
    }

    async fn edit_category(&self, category: Category) -> Result<ID> {
        todo!()
    }

    async fn delete_category(&self, id: ID) -> Result<()> {
        todo!()
    }

    async fn query_tags(&self, req: QueryTag) -> Result<Vec<Tag>> {
        todo!()
    }

    async fn add_tag(&self, tag: Tag) -> Result<ID> {
        todo!()
    }

    async fn edit_tag(&self, tag: Tag) -> Result<ID> {
        todo!()
    }

    async fn delete_tag(&self, id: ID) -> Result<()> {
        todo!()
    }

    async fn tag_to_articles(&self, tag_id: ID) -> Result<Vec<ID>> {
        let sql = "SELECT article_ids FROM blog.tag2article WHERE tag_id = $1";
        let res = sqlx::query(sql)
            .bind(tag_id)
            .fetch_one(self.deref())
            .await?;
        Ok(res.get(0))
    }

    async fn article_to_tags(&self, article_id: ID) -> Result<Vec<ID>> {
        let sql = "SELECT tag_ids FROM blog.article2tag WHERE article_id = $1";
        let res = sqlx::query(sql)
            .bind(article_id)
            .fetch_one(self.deref())
            .await?;
        Ok(res.get(0))
    }
}
