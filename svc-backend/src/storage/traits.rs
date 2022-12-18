use util_pb::{Article, Category, QueryArticle, QueryCategory, QueryTag, Tag};

use crate::Result;

pub type ID = i32;

#[tonic::async_trait]
pub trait BlogDB {
    async fn query_articles(&self, mut req: QueryArticle) -> Result<Vec<Article>>;

    async fn add_article(&self, article: Article) -> Result<ID>;

    async fn edit_article(&self, article: Article) -> Result<ID>;

    async fn delete_article(&self, id: ID) -> Result<()>;

    async fn query_categories(&self, req: QueryCategory) -> Result<Vec<Category>>;

    async fn add_category(&self, category: Category) -> Result<ID>;

    async fn edit_category(&self, category: Category) -> Result<ID>;

    async fn delete_category(&self, id: ID) -> Result<()>;

    async fn query_tags(&self, req: QueryTag) -> Result<Vec<Tag>>;

    async fn add_tag(&self, tag: Tag) -> Result<ID>;

    async fn edit_tag(&self, tag: Tag) -> Result<ID>;

    async fn delete_tag(&self, id: ID) -> Result<()>;

    async fn tag_to_articles(&self, tag_id: ID) -> Result<Vec<ID>>;

    async fn article_to_tags(&self, article_id: ID) -> Result<Vec<ID>>;
}
