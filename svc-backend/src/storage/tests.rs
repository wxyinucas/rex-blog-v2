use std::path::Path;

use sqlx::PgPool;
use sqlx_db_tester::TestPg;

use util_pb::{to_timestamp, QueryArticle};

use crate::storage::traits::BlogDB;
use crate::storage::DBPool;

async fn load_test_db() -> TestPg {
    dotenv::dotenv().ok();

    TestPg::new(
        std::env::var("TDB_URL").unwrap(),
        Path::new("../migrations"),
    )
}

#[allow(unused)]
async fn load_pool() -> PgPool {
    dotenv::dotenv().ok();

    let db_addr = std::env::var("DATABASE_URL").unwrap();
    sqlx::PgPool::connect(&db_addr).await.unwrap()
}

#[tokio::test]
async fn query_article_should_work() {
    let tdb = load_test_db().await;
    let pool = tdb.get_pool().await;
    let db = DBPool::new(pool);

    // take all
    let req = QueryArticle::default();
    let res = db.query_articles(req).await;
    assert_eq!(res.unwrap().len(), 2);

    // by one tag
    let req = QueryArticle {
        tags_id: vec![1],
        ..QueryArticle::default()
    };
    let res = db.query_articles(req).await.unwrap();
    assert_eq!(res[0].id, 1000);

    // by tags
    let req = QueryArticle {
        tags_id: vec![1, 2],
        ..QueryArticle::default()
    };
    let res = db.query_articles(req).await.unwrap();
    assert_eq!(res[0].id, 1000);

    // by time
    let this_year = chrono::Local::now();
    let req = QueryArticle {
        created_year: Some(to_timestamp(this_year)),
        ..QueryArticle::default()
    };
    let res = db.query_articles(req).await;
    assert_eq!(res.unwrap().len(), 2);

    // by title
    let req = QueryArticle {
        title: "test_title1".to_string(),
        ..QueryArticle::default()
    };
    let res = db.query_articles(req).await.unwrap();
    assert_eq!(res[0].id, 1001);
}

#[tokio::test]
async fn add_article_should_work() {
    let tdb = load_test_db().await;
    let pool = tdb.get_pool().await;
    // let pool = load_pool().await;
    let db = DBPool::new(pool);

    // add one
    let req = util_pb::Article {
        title: "test_add".to_string(),
        content: "test add".to_string(),
        tags_id: vec![1, 2],
        category_id: 1,
        ..util_pb::Article::default()
    };
    let id = db.add_article(req).await.unwrap();
    println!("{}", id);

    let req = QueryArticle {
        title: "test_add".to_string(),
        ..QueryArticle::default()
    };
    let res = db.query_articles(req).await.unwrap();
    assert_eq!(res[0].id, id);
    assert_eq!(res[0].tags_id, vec![1, 2, 0]);
    assert_eq!(res[0].summary, "test add".to_string());
}

#[tokio::test]
async fn update_article_should_work() {
    let tdb = load_test_db().await;
    let pool = tdb.get_pool().await;
    let db = DBPool::new(pool);

    let req = util_pb::Article {
        id: 1000,
        title: "test_update".to_string(),
        content: "test update".to_string(),
        tags_id: vec![1],
        category_id: 1,
        ..util_pb::Article::default()
    };
    db.edit_article(req).await.unwrap();

    let req = QueryArticle {
        title: "test_update".to_string(),
        ..QueryArticle::default()
    };
    let res = db.query_articles(req).await.unwrap();
    assert_eq!(res[0].tags_id, vec![1]);

    let req = util_pb::Article {
        id: 1000,
        tags_id: vec![2, 1],
        category_id: 1,
        ..util_pb::Article::default()
    };
    db.edit_article(req).await.unwrap();

    let req = QueryArticle {
        title: "test_update".to_string(),
        ..QueryArticle::default()
    };
    let res = db.query_articles(req).await.unwrap();
    assert_eq!(res[0].tags_id, vec![1, 2]);
    assert_eq!(res[0].summary, "test update".to_string());
}

#[tokio::test]
async fn delete_article_should_work() {
    let tdb = load_test_db().await;
    let pool = tdb.get_pool().await;
    let db = DBPool::new(pool);

    let req = util_pb::Article {
        title: "test_delete".to_string(),
        content: "test delete".to_string(),
        category_id: 1,
        tags_id: vec![1, 2],
        ..util_pb::Article::default()
    };
    let id = db.add_article(req).await.unwrap();

    let res = db.delete_article(id).await;
    assert!(res.is_ok());
}

#[tokio::test]
async fn categories_operator_should_work() {
    let tdb = load_test_db().await;
    let pool = tdb.get_pool().await;
    let db = DBPool::new(pool);

    // query
    let req = util_pb::QueryCategory::default();
    let res = db.query_categories(req).await.unwrap();
    assert_eq!(res.len(), 2);

    // add
    let req = util_pb::Category {
        name: "test_add".to_string(),
        ..util_pb::Category::default()
    };
    let new_id = db.add_category(req).await.unwrap();

    let req = util_pb::QueryCategory {
        name: "test_add".to_string(),
        ..util_pb::QueryCategory::default()
    };
    let res = db.query_categories(req).await.unwrap();
    assert_eq!(res[0].name, "test_add".to_string());

    // edit
    let req = util_pb::Category {
        id: new_id,
        name: "test_edit".to_string(),
    };
    db.edit_category(req).await.unwrap();

    let req = util_pb::QueryCategory {
        ids: vec![new_id],
        ..util_pb::QueryCategory::default()
    };
    let res = db.query_categories(req).await.unwrap();
    assert_eq!(res[0].name, "test_edit".to_string());

    // delete
    db.delete_category(new_id).await.unwrap();

    let req = util_pb::QueryCategory::default();
    let res = db.query_categories(req).await.unwrap();
    assert_eq!(res.len(), 2);
}

#[tokio::test]
async fn tags_operator_should_work() {
    let tdb = load_test_db().await;
    let pool = tdb.get_pool().await;
    let db = DBPool::new(pool);

    // query
    let req = util_pb::QueryTag::default();
    let res = db.query_tags(req).await.unwrap();
    assert_eq!(res.len(), 3);

    // add
    let req = util_pb::Tag {
        name: "test_add".to_string(),
        ..util_pb::Tag::default()
    };
    let new_id = db.add_tag(req).await.unwrap();

    let req = util_pb::QueryTag {
        name: "test_add".to_string(),
        ..util_pb::QueryTag::default()
    };
    let res = db.query_tags(req).await.unwrap();
    assert_eq!(res[0].name, "test_add".to_string());

    // edit
    let req = util_pb::Tag {
        id: new_id,
        name: "test_edit".to_string(),
    };
    db.edit_tag(req).await.unwrap();

    let req = util_pb::QueryTag {
        ids: vec![new_id],
        ..util_pb::QueryTag::default()
    };
    let res = db.query_tags(req).await.unwrap();
    assert_eq!(res[0].name, "test_edit".to_string());

    // delete
    db.delete_tag(new_id).await.unwrap();

    let req = util_pb::QueryTag::default();
    let res = db.query_tags(req).await.unwrap();
    assert_eq!(res.len(), 3);
}

#[tokio::test]
async fn tag_article_should_work() {
    let tdb = load_test_db().await;
    let pool = tdb.get_pool().await;
    let db = DBPool::new(pool);

    // delete article first
    let req = util_pb::Tag {
        name: "test_tag".to_string(),
        ..util_pb::Tag::default()
    };
    let tag_id = db.add_tag(req).await.unwrap();

    let req = util_pb::Article {
        title: "test_tag".to_string(),
        content: "test tag".to_string(),
        category_id: 1,
        tags_id: vec![tag_id],
        ..util_pb::Article::default()
    };
    let article_id = db.add_article(req).await.unwrap();

    db.delete_article(article_id).await.unwrap();
    db.delete_tag(tag_id).await.unwrap();

    // delete tag first
    let req = util_pb::Tag {
        name: "test_tag".to_string(),
        ..util_pb::Tag::default()
    };
    let tag_id = db.add_tag(req).await.unwrap();

    let req = util_pb::Article {
        title: "test_tag".to_string(),
        content: "test tag".to_string(),
        category_id: 1,
        tags_id: vec![tag_id],
        ..util_pb::Article::default()
    };
    let article_id = db.add_article(req).await.unwrap();

    db.delete_tag(tag_id).await.unwrap();

    let req = util_pb::QueryArticle {
        ids: vec![article_id],
        ..util_pb::QueryArticle::default()
    };
    let res = db.query_articles(req).await.unwrap();
    assert_eq!(res[0].tags_id, vec![0]);
}
