use axum::extract::Path;
use axum::response::Html;
use axum::Extension;
use pulldown_cmark::{Options, Parser};
use tera::Context;
use util_pb::query_request::Query;

use crate::common_handlers::{get_categories, get_tags, TeraHtml};
use crate::errors::Result;
use crate::shared_state::SharedState;

pub async fn page_index(Extension(state): Extension<SharedState>) -> Result<TeraHtml> {
    let context = tera::Context::new();
    let page = state
        .tera()
        .render("demonstration/index.html", &context)
        .unwrap();

    Ok(Html(page))
}

pub async fn page_categories(Extension(state): Extension<SharedState>) -> Result<TeraHtml> {
    let mut context = tera::Context::new();
    let categories = get_categories(&state).await;

    context.insert("categories", &categories);
    let page = state
        .tera()
        .render("demonstration/categories/categories.html", &context)
        .unwrap();

    Ok(Html(page))
}

pub async fn page_category(
    Path(category_id): Path<i32>,
    Extension(state): Extension<SharedState>,
) -> Result<TeraHtml> {
    let mut ctx = Context::new();
    let query = util_pb::QueryRequest {
        query: Some(Query::QueryArticle(util_pb::QueryArticle {
            category_id,
            ..Default::default()
        })),
    };
    let res = state.client().query(query).await?.into_inner().articles;
    ctx.insert("articles", &res);

    let page = state.tera().render("demonstration/articles.html", &ctx)?;
    Ok(Html(page))
}

pub async fn page_tags(Extension(state): Extension<SharedState>) -> Result<TeraHtml> {
    let mut ctx = Context::new();
    let tags = get_tags(&state).await;
    ctx.insert("tags", &tags);
    let page = state.tera().render("demonstration/tags/tags.html", &ctx)?;
    Ok(Html(page))
}

pub async fn page_tag(
    Path(tag_id): Path<i32>,
    Extension(state): Extension<SharedState>,
) -> Result<TeraHtml> {
    let mut ctx = Context::new();

    let query = util_pb::QueryRequest {
        query: Some(Query::QueryArticle(util_pb::QueryArticle {
            tags_id: vec![tag_id],
            ..Default::default()
        })),
    };
    let res = state.client().query(query).await?.into_inner().articles;
    ctx.insert("articles", &res);

    let page = state.tera().render("demonstration/articles.html", &ctx)?;
    Ok(Html(page))
}

pub async fn page_show_article(
    Path(article_id): Path<i32>,
    Extension(state): Extension<SharedState>,
) -> Result<TeraHtml> {
    let mut ctx = Context::new();

    let query = util_pb::QueryRequest {
        query: Some(Query::QueryArticle(util_pb::QueryArticle {
            ids: vec![article_id],
            ..Default::default()
        })),
    };
    let res = state.client().query(query).await?.into_inner().articles;
    ctx.insert("article", &res[0]);

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let mut content = String::new();
    let parser = Parser::new_ext(&res[0].content, options);
    pulldown_cmark::html::push_html(&mut content, parser);
    ctx.insert("content", &content);

    let page = state.tera().render("demonstration/article.html", &ctx)?;
    Ok(Html(page))
}
