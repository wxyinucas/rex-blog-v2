#![allow(clippy::all, unused_imports, dead_code)]

use std::collections::HashMap;

use axum::extract::Path;
use axum::response::Html;
use axum::Extension;
use tera::Context;

use util_pb::query_request::Query;
use util_pb::{ArticleState, Category, Tag};

use crate::common_handlers::{Redirect, TeraHtml};
use crate::errors::{FrontendError, Result};
use crate::shared_state::SharedState;

pub async fn page_dashboard(Extension(state): Extension<SharedState>) -> Result<TeraHtml> {
    let ctx = Context::new();

    let page = state
        .tera()
        .render("management/base.html", &ctx)
        .map_err(FrontendError::from)?;
    Ok(Html(page))
}

/* =================================================================


articles


================================================================== */

pub async fn page_article_list(Extension(state): Extension<SharedState>) -> Result<TeraHtml> {
    let mut ctx = Context::new();

    let query_article = util_pb::QueryArticle::default();
    let query = util_pb::QueryRequest {
        query: Some(Query::QueryArticle(query_article)),
    };

    let res = state.client().query(query).await?.into_inner();
    let articles = res.articles;

    let (c_map, t_map) = get_categories_tags(&state).await;

    let categories = articles
        .iter()
        .map(|article| c_map.get(&article.category_id).unwrap().clone())
        .collect::<Vec<String>>();

    let tags = articles
        .iter()
        .map(|article| {
            article
                .tags_id
                .iter()
                .map(|tag_id| t_map.get(tag_id).unwrap().clone())
                .collect::<Vec<_>>()
                .join(", ")
        })
        .collect::<Vec<String>>();

    let articles_with_name = articles
        .iter()
        .zip(categories.iter())
        .zip(tags.iter())
        .collect::<Vec<_>>();
    ctx.insert("articles_with_name", &articles_with_name);

    let page = state
        .tera()
        .render("management/articles/index.html", &ctx)
        .map_err(|err| {
            tracing::error!("render error: \n{:?}", err);
            tracing::error!("ctx: {:?}", ctx);
            <tera::Error as Into<FrontendError>>::into(err)
        })?;
    Ok(Html(page))
}

pub async fn page_article_query() -> Result<TeraHtml> {
    todo!()
}

pub async fn page_article_add(Extension(state): Extension<SharedState>) -> Result<TeraHtml> {
    let mut ctx = Context::new();

    let categories = get_categories(&state).await;
    ctx.insert("categories", &categories);

    let page = state
        .tera()
        .render("management/articles/add.html", &ctx)
        .map_err(|err| {
            tracing::info!("render error: {:?}", err);
            tracing::error!("ctx: {:?}", ctx);
            <tera::Error as Into<FrontendError>>::into(err)
        })?;
    Ok(Html(page))
}

pub async fn page_article_edit(
    Path(id): Path<i32>,
    Extension(state): Extension<SharedState>,
) -> Result<TeraHtml> {
    let mut ctx = Context::new();

    let categories = get_categories(&state).await;
    let article_states = articles_states();
    ctx.insert("article_states", &article_states);
    ctx.insert("categories", &categories);

    let query = util_pb::QueryRequest {
        query: Some(Query::QueryArticle(util_pb::QueryArticle {
            ids: vec![id],
            ..Default::default()
        })),
    };
    let mut res = state.client().query(query).await?.into_inner();
    let article = res.articles.pop().unwrap();
    ctx.insert("article", &article);

    let (_, t_map) = get_categories_tags(&state).await;
    let tags_name = article
        .tags_id
        .iter()
        .map(|tag_id| t_map.get(tag_id).unwrap().clone())
        .collect::<Vec<_>>()
        .join(", ");
    ctx.insert("tags_name", &tags_name);

    let page = state
        .tera()
        .render("management/articles/edit.html", &ctx)
        .map_err(|err| {
            tracing::info!("render error: {:?}", err);
            tracing::error!("ctx: {:?}", ctx);
            <tera::Error as Into<FrontendError>>::into(err)
        })?;
    Ok(Html(page))
}

/* =================================================================


categories


================================================================== */
pub async fn page_category_list(Extension(state): Extension<SharedState>) -> Result<TeraHtml> {
    let mut ctx = Context::new();

    let categories = get_categories(&state).await;
    ctx.insert("categories", &categories);

    let page = state
        .tera()
        .render("management/categories/index.html", &ctx)
        .map_err(FrontendError::from)
        .unwrap();
    Ok(Html(page))
}

pub async fn page_category_query(Extension(_state): Extension<SharedState>) -> Result<TeraHtml> {
    todo!()
}

pub async fn page_category_add(Extension(state): Extension<SharedState>) -> Result<TeraHtml> {
    let ctx = Context::new();

    let page = state
        .tera()
        .render("management/categories/add.html", &ctx)
        .unwrap();

    Ok(Html(page))
}

pub async fn page_category_edit(
    Path(id): Path<i32>,
    Extension(state): Extension<SharedState>,
) -> Result<TeraHtml> {
    let mut ctx = Context::new();

    let query = util_pb::QueryRequest {
        query: Some(Query::QueryCategory(util_pb::QueryCategory {
            ids: vec![id],
            ..Default::default()
        })),
    };
    let category = state
        .client()
        .query(query)
        .await?
        .into_inner()
        .categories
        .pop()
        .unwrap();
    ctx.insert("category", &category);

    let page = state
        .tera()
        .render("management/categories/edit.html", &ctx)
        .unwrap();
    Ok(Html(page))
}

/* =================================================================


tags


================================================================== */
pub async fn page_tag_list(Extension(state): Extension<SharedState>) -> Result<TeraHtml> {
    let mut ctx = Context::new();

    let tags = get_tags(&state).await;
    ctx.insert("tags", &tags);

    let page = state
        .tera()
        .render("management/tags/index.html", &ctx)
        .map_err(FrontendError::from)
        .unwrap();
    Ok(Html(page))
}

pub async fn page_tag_query() -> Result<TeraHtml> {
    todo!()
}

pub async fn page_tag_add(Extension(state): Extension<SharedState>) -> Result<TeraHtml> {
    let ctx = Context::new();

    let page = state
        .tera()
        .render("management/tags/add.html", &ctx)
        .unwrap();
    Ok(Html(page))
}

pub async fn page_tag_edit(
    Path(id): Path<i32>,
    Extension(state): Extension<SharedState>,
) -> Result<TeraHtml> {
    let mut ctx = Context::new();

    let query = util_pb::QueryRequest {
        query: Some(Query::QueryTag(util_pb::QueryTag {
            ids: vec![id],
            ..Default::default()
        })),
    };
    let tag = state
        .client()
        .query(query)
        .await?
        .into_inner()
        .tags
        .pop()
        .unwrap();
    ctx.insert("tag", &tag);

    let page = state
        .tera()
        .render("management/tags/edit.html", &ctx)
        .unwrap();
    Ok(Html(page))
}

/* =================================================================


utils


================================================================== */
async fn get_categories_tags(state: &SharedState) -> (HashMap<i32, String>, HashMap<i32, String>) {
    // categories
    let query_category = util_pb::QueryCategory::default();
    let query = util_pb::QueryRequest {
        query: Some(Query::QueryCategory(query_category)),
    };
    let res = state.client().query(query).await.unwrap().into_inner();
    let categories = res.categories;

    let mut c_map = HashMap::new();
    for c in categories {
        c_map.insert(c.id, c.name);
    }

    // tags
    let query_tag = util_pb::QueryTag::default();
    let query = util_pb::QueryRequest {
        query: Some(Query::QueryTag(query_tag)),
    };
    let res = state.client().query(query).await.unwrap().into_inner();
    let tags = res.tags;

    let mut t_map = HashMap::new();
    for t in tags {
        t_map.insert(t.id, t.name);
    }

    (c_map, t_map)
}

async fn get_categories(state: &SharedState) -> Vec<Category> {
    let query_category = util_pb::QueryCategory::default();
    let query = util_pb::QueryRequest {
        query: Some(Query::QueryCategory(query_category)),
    };
    let res = state.client().query(query).await.unwrap().into_inner();
    res.categories
}

async fn get_tags(state: &SharedState) -> Vec<Tag> {
    let query_tag = util_pb::QueryTag::default();
    let query = util_pb::QueryRequest {
        query: Some(Query::QueryTag(query_tag)),
    };
    let res = state.client().query(query).await.unwrap().into_inner();
    res.tags
}

fn articles_states() -> Vec<&'static str> {
    vec![
        ArticleState::Published.as_str_name(),
        ArticleState::Hidden.as_str_name(),
    ]
}
