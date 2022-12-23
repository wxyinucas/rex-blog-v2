use axum::http::StatusCode;
use axum::routing::get_service;
use axum::{Extension, Router};
use page_frontend::{demonstration_router, management_router};
use tera::Tera;
use tower_http::services::ServeDir;

use page_frontend::shared_state::SharedState;
use util_pb::blog_service_client::BlogServiceClient;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt().pretty().init();

    let backend_url = std::env::var("BACKEND_URL").unwrap();
    let frontend_url = std::env::var("FRONTEND_URL").unwrap();

    let tera = Tera::new("page-frontend/templates/**/*.html").unwrap();
    let client = BlogServiceClient::connect(format!("http://{}", backend_url))
        .await
        .unwrap();

    let shared_state = SharedState::new(tera, client);
    let static_svc = get_service(ServeDir::new("page-frontend/templates/assets")).handle_error(
        |err| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Static resource：{:?}", err),
            )
        },
    );

    let app = Router::new()
        .nest("/", demonstration_router())
        .nest("/management", management_router())
        .layer(Extension(shared_state))
        .nest_service("/assets", static_svc);

    tracing::info!("FRONTEND_URL listening on: {}", frontend_url);
    axum::Server::bind(&frontend_url.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
