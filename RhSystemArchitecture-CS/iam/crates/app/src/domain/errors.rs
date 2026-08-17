//! Domain errors for the iam bounded context.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum IamError {
    #[error("user not found")]
    NotFound,
    #[error("username already taken")]
    DuplicateUsername,
    #[error("invalid status transition")]
    InvalidTransition,
    #[error("validation error: {0}")]
    Validation(String),
}
