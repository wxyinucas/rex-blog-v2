use crate::demonstration::handlers_pages::*;
use axum::routing::get;
use axum::Router;

pub fn demonstration_router() -> Router {
    Router::new()
        .route("/", get(page_index))
        .route("/categories", get(page_categories))
        .route("/categories/:category_id", get(page_category))
        .route("/tags", get(page_tags))
        .route("/tags/:tag_id", get(page_tag))
        .route("/articles/:article_id", get(page_show_article))
}
