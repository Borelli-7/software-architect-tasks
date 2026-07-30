//! Domain errors for the training-program bounded context.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TrainingError {
    #[error("program not found")]
    NotFound,
    #[error("program at capacity")]
    CapacityExceeded,
    #[error("employee already enrolled")]
    DuplicateEnrollment,
    #[error("invalid status transition")]
    InvalidTransition,
    #[error("validation error: {0}")]
    Validation(String),
}
