use axum::headers::HeaderMap;
use axum::http::StatusCode;
use axum::response::Html;
use futures::future;
use tonic::transport::Channel;
use tonic::Request;

use util_pb::blog_service_client::BlogServiceClient;
use util_pb::create_request::Create;
use util_pb::query_request::Query;
use util_pb::{ArticleState, Category, CreateRequest, QueryTag, Tag};

use crate::errors::{FrontendError, Result};
use crate::shared_state::SharedState;

pub type Redirect = (StatusCode, HeaderMap);
pub type TeraHtml = Html<String>;

pub(crate) fn redirect_with_cookies(url: &str, cookies: Option<&str>) -> Redirect {
    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::LOCATION,
        url.parse().expect("URL parse failed"),
    );

    if let Some(cookies) = cookies {
        headers.insert(axum::http::header::SET_COOKIE, cookies.parse().unwrap());
        // TODO: 多个 cookies 可以解析吗？
    }
    (StatusCode::FOUND, headers)
}

pub async fn create_if_not_exists_then_return_tag_id(
    tag_name: String,
    mut client: BlogServiceClient<Channel>,
) -> Result<i32> {
    let query = QueryTag {
        ids: vec![],
        name: tag_name.to_string(),
    };
    let tonic_req = Request::new(util_pb::QueryRequest {
        query: Some(Query::QueryTag(query)),
    });
    let tonic_res = client.query(tonic_req).await?;
    let mut res = tonic_res.into_inner().tags;

    if res.len() == 1 {
        Ok(res.pop().unwrap().id)
    } else if res.is_empty() {
        let create = Tag {
            name: tag_name.to_string(),
            ..Tag::default()
        };
        let tonic_req = Request::new(CreateRequest {
            create: Some(Create::Tag(create)),
        });
        let tonic_res = client.create(tonic_req).await?;
        Ok(tonic_res.into_inner().id)
    } else {
        let msg = format!("Unexpected tag response length: {}", res.len());
        tracing::error!("{}", &msg);
        Err(FrontendError::InternalError(msg))
    }
}

pub async fn get_ids_from_tag_str(tags_str: &str, state: &SharedState) -> Result<Vec<i32>> {
    let tag_client = state.client();

    let tag_ids = future::try_join_all(
        tags_str
            .split(',')
            .map(|s| s.trim().to_string())
            .map(|tag| create_if_not_exists_then_return_tag_id(tag, tag_client.clone())),
    )
    .await?;
    Ok(tag_ids)
}

/* =================================================================


get all


================================================================== */
pub async fn get_categories(state: &SharedState) -> Vec<Category> {
    let query_category = util_pb::QueryCategory::default();
    let query = util_pb::QueryRequest {
        query: Some(Query::QueryCategory(query_category)),
    };
    let res = state.client().query(query).await.unwrap().into_inner();
    res.categories
}

pub async fn get_tags(state: &SharedState) -> Vec<Tag> {
    let query_tag = util_pb::QueryTag::default();
    let query = util_pb::QueryRequest {
        query: Some(Query::QueryTag(query_tag)),
    };
    let res = state.client().query(query).await.unwrap().into_inner();
    res.tags
}

pub fn articles_states() -> Vec<&'static str> {
    vec![
        ArticleState::Published.as_str_name(),
        ArticleState::Hidden.as_str_name(),
    ]
}
