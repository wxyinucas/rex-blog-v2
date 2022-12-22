#![allow(clippy::all, unused_imports, dead_code)]

use axum::headers::HeaderMap;
use axum::http::StatusCode;
use axum::response::Html;

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
