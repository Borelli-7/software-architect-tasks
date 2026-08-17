//! Domain errors for the career-path-planning bounded context.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CareerError {
    #[error("career path not found")]
    NotFound,
    #[error("invalid career step ordering")]
    InvalidStepOrder,
    #[error("validation error: {0}")]
    Validation(String),
}
