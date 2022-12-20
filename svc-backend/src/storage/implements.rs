use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::ops::Deref;

use sqlx::Row;

use util_pb::transfer::AS;
use util_pb::{
    get_summary, to_timestamp, transfer::ToSql, Article, ArticleState, Category, QueryArticle,
    QueryCategory, QueryTag, Tag,
};

use crate::error::Result;
use crate::storage::traits::{BlogDB, ID};
use crate::storage::DBPool;
use crate::BackendError;

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

        let res = sqlx::query_as::<_, Article>(&sql)
            .fetch_all(self.deref())
            .await?;
        Ok(res)
    }

    async fn add_article(&self, article: Article) -> Result<ID> {
        let summary = if article.summary.is_empty() {
            get_summary(&article.content)
        } else {
            article.summary
        };

        // step1: bolg.articles
        let sql = "INSERT INTO blog.articles (title, content, summary, state, category_id) VALUES ($1, $2, $3, $4, $5) RETURNING id;";
        let state = AS::try_from(article.state).unwrap();

        let id = sqlx::query(sql)
            .bind(&article.title)
            .bind(&article.content)
            .bind(summary)
            .bind(state)
            .bind(article.category_id)
            .fetch_one(self.deref())
            .await?
            .get(0);

        // step2: blog.article_tag
        let sql = "INSERT INTO blog.article_tag (article_id, tag_id) VALUES ($1, $2);";
        let mut tags_id = article.tags_id;
        tags_id.push(0);
        for tag_id in tags_id {
            sqlx::query(sql)
                .bind(id)
                .bind(tag_id)
                .execute(self.deref())
                .await?;
        }
        Ok(id)
    }

    async fn edit_article(&self, article: Article) -> Result<ID> {
        let tags_id = article.tags_id.clone();
        let article_id = article.id;
        // step1
        let kv_pairs_without_tags = article.to_sql();
        if kv_pairs_without_tags.is_empty() {
            return Err(BackendError::InvalidRequest(
                "Empty update article request.".to_string(),
            ));
        }

        let sql = format!(
            "UPDATE blog.articles SET {} WHERE id = $1 RETURNING id;",
            kv_pairs_without_tags
        );
        let id = sqlx::query(&sql)
            .bind(article_id)
            .fetch_one(self.deref())
            .await?
            .get(0);

        // step2
        let old_tags = self.article_to_tags(article_id).await?;
        let old_tags = HashSet::<_>::from_iter(old_tags);
        let new_tags = HashSet::<_>::from_iter(tags_id);

        let need_add_tags = new_tags
            .difference(&old_tags)
            .cloned()
            .collect::<Vec<i32>>();
        let need_delete_tags = old_tags
            .difference(&new_tags)
            .cloned()
            .collect::<Vec<i32>>();

        let sql = "INSERT INTO blog.article_tag (article_id, tag_id) VALUES ($1, $2);";
        for tag_id in need_add_tags {
            sqlx::query(sql)
                .bind(id)
                .bind(tag_id)
                .execute(self.deref())
                .await?;
        }

        let sql = "DELETE FROM blog.article_tag WHERE article_id = $1 AND tag_id = $2;";
        for tag_id in need_delete_tags {
            sqlx::query(sql)
                .bind(id)
                .bind(tag_id)
                .execute(self.deref())
                .await?;
        }
        Ok(id)
    }

    async fn delete_article(&self, id: ID) -> Result<()> {
        // step2
        let sql = "DELETE FROM blog.article_tag WHERE article_id = $1;";
        sqlx::query(sql).bind(id).execute(self.deref()).await?;

        // step1
        let sql = "DELETE FROM blog.articles WHERE id = $1;";
        sqlx::query(sql).bind(id).execute(self.deref()).await?;
        Ok(())
    }

    async fn query_categories(&self, req: QueryCategory) -> Result<Vec<Category>> {
        let condition = req.to_sql();
        let sql = format!("SELECT * FROM blog.categories WHERE {}", condition);
        let res = sqlx::query_as::<_, Category>(&sql)
            .fetch_all(self.deref())
            .await?;
        Ok(res)
    }

    async fn add_category(&self, category: Category) -> Result<ID> {
        let sql = "INSERT INTO blog.categories (name) VALUES ($1) RETURNING id;";
        let id = sqlx::query(sql)
            .bind(&category.name)
            .fetch_one(self.deref())
            .await?
            .get(0);
        Ok(id)
    }

    async fn edit_category(&self, category: Category) -> Result<ID> {
        let sql = "UPDATE blog.categories SET name = $1 WHERE id = $2 RETURNING id;";
        let id = sqlx::query(sql)
            .bind(&category.name)
            .bind(category.id)
            .fetch_one(self.deref())
            .await?
            .get(0);
        Ok(id)
    }

    async fn delete_category(&self, id: ID) -> Result<()> {
        let sql = "Delete FROM blog.categories WHERE id = $1 RETURNING id;";
        sqlx::query(sql).bind(id).execute(self.deref()).await?;
        Ok(())
    }

    async fn query_tags(&self, req: QueryTag) -> Result<Vec<Tag>> {
        let condition = req.to_sql();
        let sql = format!("SELECT * FROM blog.tags WHERE {}", condition);
        let res = sqlx::query_as::<_, Tag>(&sql)
            .fetch_all(self.deref())
            .await?;
        Ok(res)
    }

    async fn add_tag(&self, tag: Tag) -> Result<ID> {
        let sql = "INSERT INTO blog.tags (name) VALUES ($1) RETURNING id;";
        let id = sqlx::query(sql)
            .bind(&tag.name)
            .fetch_one(self.deref())
            .await?
            .get(0);
        Ok(id)
    }

    async fn edit_tag(&self, tag: Tag) -> Result<ID> {
        let sql = "UPDATE blog.tags SET name = $1 WHERE id = $2 RETURNING id;";
        let id = sqlx::query(sql)
            .bind(&tag.name)
            .bind(tag.id)
            .fetch_one(self.deref())
            .await?
            .get(0);
        Ok(id)
    }

    async fn delete_tag(&self, id: ID) -> Result<()> {
        // step2
        let sql = "DELETE FROM blog.article_tag WHERE tag_id = $1;";
        sqlx::query(sql).bind(id).execute(self.deref()).await?;

        // step1
        let sql = "Delete FROM blog.tags WHERE id = $1 RETURNING id;";
        sqlx::query(sql).bind(id).execute(self.deref()).await?;
        Ok(())
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
        let article_ids: Vec<i32> = res.get(0);
        return if article_ids.is_empty() {
            Ok(vec![0])
        } else {
            Ok(article_ids)
        };
    }
}
