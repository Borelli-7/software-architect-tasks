//! Client-side error type for gateway calls.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("transport error: {0}")]
    Transport(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("not found")]
    NotFound,
    #[error("server returned {status}: {message}")]
    Status { status: u16, message: String },
    #[error("failed to decode response: {0}")]
    Decode(String),
}

impl From<reqwest::Error> for ApiError {
    fn from(e: reqwest::Error) -> Self {
        ApiError::Transport(e.to_string())
    }
}
