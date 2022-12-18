use thiserror::Error;

pub type Result<T> = std::result::Result<T, BackendError>;

#[derive(Error, Debug)]
pub enum BackendError {
    #[error("Sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),
}
