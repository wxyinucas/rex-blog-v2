#![allow(clippy::all, unused_imports, dead_code)]

use axum::extract::Path;
use axum::{Extension, Form};
use tonic::Request;

use util_pb::create_request::Create;
use util_pb::delete_request::Delete;
use util_pb::update_request::Update;
use util_pb::{Article, CreateRequest, DeleteRequest, UpdateRequest};

use crate::common_handlers::{get_ids_from_tag_str, redirect_with_cookies, Redirect};
use crate::errors::{FrontendError, Result};
use crate::management::forms::{FormArticle, FormCategory, FormTag};
use crate::shared_state::SharedState;

/* =================================================================


articles


================================================================== */

pub async fn handler_article_query() -> Result<Redirect> {
    todo!()
}

pub async fn handler_article_add(
    Extension(state): Extension<SharedState>,
    Form(form_article): Form<FormArticle>,
) -> Result<Redirect> {
    let tags_id = get_ids_from_tag_str(&form_article.tags, &state).await?;

    let article = Article {
        id: 0,
        title: form_article.title,
        content: form_article.content,
        summary: form_article.summary,
        state: form_article.state,
        created_at: None,
        updated_at: None,
        category_id: form_article.category_id,
        tags_id,
    };

    let req = Request::new(CreateRequest {
        create: Some(Create::Article(article)),
    });
    let res = state.client().create(req).await?.into_inner();
    Ok(redirect_with_cookies(
        &format!("/management/articles?msg=create article with id {}", res.id),
        None,
    ))
}

pub async fn handler_article_edit(
    Path(id): Path<i32>,
    Extension(state): Extension<SharedState>,
    Form(form_article): Form<FormArticle>,
) -> Result<Redirect> {
    let tags_id = get_ids_from_tag_str(&form_article.tags, &state).await?;

    let article = Article {
        id,
        title: form_article.title,
        content: form_article.content,
        summary: form_article.summary,
        state: form_article.state,
        created_at: None,
        updated_at: None,
        category_id: form_article.category_id,
        tags_id,
    };
    let req = Request::new(UpdateRequest {
        update: Some(Update::Article(article)),
    });
    let res = state.client().update(req).await?.into_inner();
    Ok(redirect_with_cookies(
        &format!("/management/articles?msg=edit article with id {}", res.id),
        None,
    ))
}

pub async fn handler_article_delete(
    Path(id): Path<i32>,
    Extension(state): Extension<SharedState>,
) -> Result<Redirect> {
    let req = Request::new(DeleteRequest {
        delete: Some(Delete::ArticleId(id)),
    });
    let res = state.client().delete(req).await?.into_inner();
    Ok(redirect_with_cookies(
        &format!("/management/articles?msg=delete article with id {}", res.id),
        None,
    ))
}

/* =================================================================


categories


================================================================== */
pub async fn handler_category_query() -> Result<Redirect> {
    todo!()
}

pub async fn handler_category_add(
    Extension(state): Extension<SharedState>,
    Form(form_category): Form<FormCategory>,
) -> Result<Redirect> {
    let category = util_pb::Category {
        id: 0,
        name: form_category.name,
    };

    let req = Request::new(CreateRequest {
        create: Some(Create::Category(category)),
    });
    let res = state.client().create(req).await?.into_inner();
    Ok(redirect_with_cookies(
        &format!(
            "/management/categories?msg=create category with id {}",
            res.id
        ),
        None,
    ))
}

pub async fn handler_category_edit(
    Path(id): Path<i32>,
    Extension(state): Extension<SharedState>,
    Form(form_category): Form<FormCategory>,
) -> Result<Redirect> {
    let category = util_pb::Category {
        id,
        name: form_category.name,
    };
    let req = Request::new(UpdateRequest {
        update: Some(Update::Category(category)),
    });
    let res = state.client().update(req).await?.into_inner();
    Ok(redirect_with_cookies(
        &format!(
            "/management/categories?msg=edit category with id {}",
            res.id
        ),
        None,
    ))
}

pub async fn handler_category_delete(
    Path(id): Path<i32>,
    Extension(state): Extension<SharedState>,
) -> Result<Redirect> {
    let req = Request::new(DeleteRequest {
        delete: Some(Delete::CategoryId(id)),
    });
    let res = state.client().delete(req).await?.into_inner();
    Ok(redirect_with_cookies(
        &format!(
            "/management/categories?msg=delete category with id {}",
            res.id
        ),
        None,
    ))
}
/* =================================================================


tags


================================================================== */
pub async fn handler_tag_query() -> Result<Redirect> {
    todo!()
}

pub async fn handler_tag_add(
    Extension(state): Extension<SharedState>,
    Form(form_tag): Form<FormTag>,
) -> Result<Redirect> {
    let tag = util_pb::Tag {
        id: 0,
        name: form_tag.name,
    };

    let req = Request::new(CreateRequest {
        create: Some(Create::Tag(tag)),
    });
    let res = state.client().create(req).await?.into_inner();
    Ok(redirect_with_cookies(
        &format!("/management/tags?msg=create tag with id {}", res.id),
        None,
    ))
}

pub async fn handler_tag_edit(
    Path(id): Path<i32>,
    Extension(state): Extension<SharedState>,
    Form(form_tag): Form<FormTag>,
) -> Result<Redirect> {
    let tag = util_pb::Tag {
        id,
        name: form_tag.name,
    };
    let req = Request::new(UpdateRequest {
        update: Some(Update::Tag(tag)),
    });
    let res = state.client().update(req).await?.into_inner();
    Ok(redirect_with_cookies(
        &format!("/management/tags?msg=edit tag with id {}", res.id),
        None,
    ))
}

pub async fn handler_tag_delete(
    Path(id): Path<i32>,
    Extension(state): Extension<SharedState>,
) -> Result<Redirect> {
    let req = Request::new(DeleteRequest {
        delete: Some(Delete::TagId(id)),
    });
    let res = state.client().delete(req).await?.into_inner();
    Ok(redirect_with_cookies(
        &format!("/management/tags?msg=delete tag with id {}", res.id),
        None,
    ))
}
