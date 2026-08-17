//! Domain errors for the performance-evaluation bounded context.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum EvaluationError {
    #[error("campaign not found")]
    NotFound,
    #[error("invalid status transition")]
    InvalidTransition,
    #[error("review incomplete")]
    ReviewIncomplete,
    #[error("validation error: {0}")]
    Validation(String),
}
