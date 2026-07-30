//! Domain errors for the document bounded context.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DocumentError {
    #[error("document not found")]
    NotFound,
    #[error("unsupported content type")]
    UnsupportedContentType,
    #[error("storage failure")]
    StorageFailure,
    #[error("validation error: {0}")]
    Validation(String),
}
