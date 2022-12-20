use tonic::{Request, Response, Status};

use util_pb::blog_service_server::BlogService;
use util_pb::create_request::Create;
use util_pb::delete_request::Delete;
use util_pb::query_request::Query;
use util_pb::update_request::Update;
use util_pb::{
    CreateRequest, CreateResponse, DeleteRequest, DeleteResponse, QueryRequest, QueryResponse,
    UpdateRequest, UpdateResponse,
};

use crate::service::BackendInnerService;
use crate::storage::BlogDB;

#[tonic::async_trait]
impl BlogService for BackendInnerService {
    async fn query(
        &self,
        request: Request<QueryRequest>,
    ) -> Result<Response<QueryResponse>, Status> {
        let req = request.into_inner();
        match req.query.unwrap() {
            Query::QueryArticle(qa) => {
                let res = self.db_pool.query_articles(qa).await?;
                Ok(Response::new(QueryResponse {
                    articles: res,
                    ..QueryResponse::default()
                }))
            }
            Query::QueryCategory(qc) => {
                let res = self.db_pool.query_categories(qc).await?;
                Ok(Response::new(QueryResponse {
                    categories: res,
                    ..QueryResponse::default()
                }))
            }
            Query::QueryTag(qt) => {
                let res = self.db_pool.query_tags(qt).await?;
                Ok(Response::new(QueryResponse {
                    tags: res,
                    ..QueryResponse::default()
                }))
            }
        }
    }

    async fn create(
        &self,
        request: Request<CreateRequest>,
    ) -> Result<Response<CreateResponse>, Status> {
        let req = request.into_inner();
        match req.create.unwrap() {
            Create::Article(ca) => {
                let res = self.db_pool.add_article(ca).await?;
                Ok(Response::new(CreateResponse { id: res }))
            }
            Create::Category(cc) => {
                let res = self.db_pool.add_category(cc).await?;
                Ok(Response::new(CreateResponse { id: res }))
            }
            Create::Tag(ct) => {
                let res = self.db_pool.add_tag(ct).await?;
                Ok(Response::new(CreateResponse { id: res }))
            }
        }
    }

    async fn update(
        &self,
        request: Request<UpdateRequest>,
    ) -> Result<Response<UpdateResponse>, Status> {
        let req = request.into_inner();
        match req.update.unwrap() {
            Update::Article(ua) => {
                let res = self.db_pool.edit_article(ua).await?;
                Ok(Response::new(UpdateResponse { id: res }))
            }
            Update::Category(uc) => {
                let res = self.db_pool.edit_category(uc).await?;
                Ok(Response::new(UpdateResponse { id: res }))
            }
            Update::Tag(ut) => {
                let res = self.db_pool.edit_tag(ut).await?;
                Ok(Response::new(UpdateResponse { id: res }))
            }
        }
    }

    async fn delete(
        &self,
        request: Request<DeleteRequest>,
    ) -> Result<Response<DeleteResponse>, Status> {
        let req = request.into_inner();
        match req.delete.unwrap() {
            Delete::ArticleId(id) => {
                self.db_pool.delete_article(id).await?;
                Ok(Response::new(DeleteResponse { id }))
            }
            Delete::CategoryId(id) => {
                self.db_pool.delete_category(id).await?;
                Ok(Response::new(DeleteResponse { id }))
            }
            Delete::TagId(id) => {
                self.db_pool.delete_tag(id).await?;
                Ok(Response::new(DeleteResponse { id }))
            }
        }
    }
}
