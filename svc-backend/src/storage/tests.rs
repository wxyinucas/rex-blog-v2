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
    assert_eq!(res.len(), 1);

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
    let res = db.query_articles(req).await;
    assert_eq!(res.unwrap().len(), 1);
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
    assert_eq!(res[0].tags_id, vec![1, 2]);
}
