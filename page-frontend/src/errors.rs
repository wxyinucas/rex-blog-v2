use axum::response::{IntoResponse, Response};
use thiserror::Error;
use tonic::Status;

use svc_backend::BackendError;

pub type Result<T> = std::result::Result<T, FrontendError>;

#[derive(Error, Debug)]
pub enum FrontendError {
    #[error("Backend Error: {0}")]
    BackendError(#[from] BackendError),

    #[error("Tera Error: {0}")]
    TeraError(#[from] tera::Error),

    #[error("Status code: {0}")]
    StatusCode(String),
}

impl From<Status> for FrontendError {
    fn from(status_code: Status) -> Self {
        Self::StatusCode(status_code.to_string())
    }
}

impl IntoResponse for FrontendError {
    fn into_response(self) -> Response {
        self.to_string().into_response()
    }
}
