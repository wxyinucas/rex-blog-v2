use sqlx::PgPool;

use svc_backend::{BackendInnerService, DBPool};
use util_pb::blog_service_server::BlogServiceServer;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt().pretty().init();

    let db_addr = std::env::var("DATABASE_URL").unwrap();
    let pg_pool = PgPool::connect(&db_addr).await.unwrap();
    let inner_svc = BackendInnerService::new(DBPool::new(pg_pool));

    let addr = std::env::var("BACKEND_URL").unwrap();
    let svc = BlogServiceServer::new(inner_svc);

    tracing::info!("Services starting at: {}", addr);
    tonic::transport::Server::builder()
        .add_service(svc)
        .serve(addr.parse().unwrap())
        .await
        .unwrap();
}
