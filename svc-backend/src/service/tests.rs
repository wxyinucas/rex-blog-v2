use std::path::Path;

use sqlx_db_tester::TestPg;
use tonic::Request;

use util_pb::blog_service_server::BlogService;
use util_pb::create_request::Create;
use util_pb::delete_request::Delete;
use util_pb::query_request::Query;
use util_pb::update_request::Update;
use util_pb::{
    Article, CreateRequest, DeleteRequest, QueryArticle, QueryCategory, QueryRequest, UpdateRequest,
};

use crate::service::BackendInnerService;
use crate::storage::DBPool;

async fn load_test_db() -> TestPg {
    dotenv::dotenv().ok();

    TestPg::new(
        std::env::var("TDB_URL").unwrap(),
        Path::new("../migrations"),
    )
}

#[tokio::test]
async fn article_service_should_work() {
    let tdb = load_test_db().await;
    let pool = tdb.get_pool().await;
    let db = DBPool::new(pool);

    let inner_service = BackendInnerService::new(db);

    // add
    let article = Article {
        title: "test_add".to_string(),
        content: "test content".to_string(),
        category_id: 1,
        ..Article::default()
    };
    let add = CreateRequest {
        create: Some(Create::Article(article)),
    };
    let req = Request::new(add);
    let res = inner_service.create(req).await;
    assert!(res.is_ok());
    let id = res.unwrap().into_inner().id;

    // query
    let query = QueryRequest {
        query: Some(Query::QueryArticle(QueryArticle::default())),
    };
    let req = Request::new(query);
    let res = inner_service.query(req).await;
    assert!(res.is_ok());

    // edit
    let article = Article {
        id,
        title: "test_edit".to_string(),
        content: "test content".to_string(),
        ..Article::default()
    };
    let edit = UpdateRequest {
        update: Some(Update::Article(article)),
    };
    let req = Request::new(edit);
    let res = inner_service.update(req).await;
    assert!(res.is_ok());

    // delete
    let delete = DeleteRequest {
        delete: Some(Delete::ArticleId(id)),
    };
    let req = Request::new(delete);
    let res = inner_service.delete(req).await;
    assert!(res.is_ok());
}

#[tokio::test]
async fn category_service_should_work() {
    let tdb = load_test_db().await;
    let pool = tdb.get_pool().await;
    let db = DBPool::new(pool);

    let inner_service = BackendInnerService::new(db);

    // add
    let category = util_pb::Category {
        name: "test_add".to_string(),
        ..util_pb::Category::default()
    };
    let add = CreateRequest {
        create: Some(Create::Category(category)),
    };
    let req = Request::new(add);
    let res = inner_service.create(req).await;
    assert!(res.is_ok());
    let id = res.unwrap().into_inner().id;

    // query
    let query = QueryRequest {
        query: Some(Query::QueryCategory(QueryCategory::default())),
    };
    let req = Request::new(query);
    let res = inner_service.query(req).await;
    assert!(res.is_ok());

    // edit
    let category = util_pb::Category {
        id,
        name: "test_edit".to_string(),
    };
    let edit = UpdateRequest {
        update: Some(Update::Category(category)),
    };
    let req = Request::new(edit);
    let res = inner_service.update(req).await;
    assert!(res.is_ok());

    // delete
    let delete = DeleteRequest {
        delete: Some(Delete::CategoryId(id)),
    };
    let req = Request::new(delete);
    let res = inner_service.delete(req).await;
    assert!(res.is_ok());
}

#[tokio::test]
async fn tag_service_should_work() {
    let tdb = load_test_db().await;
    let pool = tdb.get_pool().await;
    let db = DBPool::new(pool);

    let inner_service = BackendInnerService::new(db);

    // add
    let tag = util_pb::Tag {
        name: "test_add".to_string(),
        ..util_pb::Tag::default()
    };
    let add = CreateRequest {
        create: Some(Create::Tag(tag)),
    };
    let req = Request::new(add);
    let res = inner_service.create(req).await;
    assert!(res.is_ok());
    let id = res.unwrap().into_inner().id;

    // query
    let query = QueryRequest {
        query: Some(Query::QueryTag(util_pb::QueryTag::default())),
    };
    let req = Request::new(query);
    let res = inner_service.query(req).await;
    assert!(res.is_ok());

    // edit
    let tag = util_pb::Tag {
        id,
        name: "test_edit".to_string(),
    };
    let edit = UpdateRequest {
        update: Some(Update::Tag(tag)),
    };
    let req = Request::new(edit);
    let res = inner_service.update(req).await;
    assert!(res.is_ok());

    // delete
    let delete = DeleteRequest {
        delete: Some(Delete::TagId(id)),
    };
    let req = Request::new(delete);
    let res = inner_service.delete(req).await;
    assert!(res.is_ok());
}
