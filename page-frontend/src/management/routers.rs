use axum::routing::get;
use axum::Router;

use crate::management::handlers_logic::*;
use crate::management::handlers_pages::*;

pub fn management_router() -> Router {
    let article_router = Router::new()
        .route("/", get(page_article_list))
        .route(
            "/query",
            get(page_article_query).post(handler_article_query),
        )
        .route("/add", get(page_article_add).post(handler_article_add))
        .route(
            "/edit/:id",
            get(page_article_edit).post(handler_article_edit),
        )
        .route("/delete/:id", get(handler_article_delete));

    let category_router = Router::new()
        .route("/", get(page_category_list))
        .route(
            "/query",
            get(page_category_query).post(handler_category_query),
        )
        .route("/add", get(page_category_add).post(handler_category_add))
        .route(
            "/edit/:id",
            get(page_category_edit).post(handler_category_edit),
        )
        .route("/delete/:id", get(handler_category_delete));

    let tag_router = Router::new()
        .route("/", get(page_tag_list))
        .route("/query", get(page_tag_query).post(handler_tag_query))
        .route("/add", get(page_tag_add).post(handler_tag_add))
        .route("/edit/:id", get(page_tag_edit).post(handler_tag_edit))
        .route("/delete/:id", get(handler_tag_delete));

    Router::new()
        .route("/", get(page_dashboard))
        .nest("/articles", article_router)
        .nest("/categories", category_router)
        .nest("/tags", tag_router)
}
