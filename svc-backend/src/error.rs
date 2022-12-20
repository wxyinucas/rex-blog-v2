use thiserror::Error;

pub type Result<T> = std::result::Result<T, BackendError>;

#[derive(Error, Debug)]
pub enum BackendError {
    #[error("Sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),
}

impl From<BackendError> for tonic::Status {
    fn from(e: BackendError) -> Self {
        match e {
            BackendError::SqlxError(e) => tonic::Status::internal(e.to_string()),
            BackendError::InvalidRequest(e) => tonic::Status::invalid_argument(e),
        }
    }
}
