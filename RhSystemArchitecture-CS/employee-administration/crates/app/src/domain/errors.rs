//! Domain errors for the employee bounded context.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum EmployeeError {
    #[error("employee not found")]
    NotFound,
    #[error("invalid status transition")]
    InvalidTransition,
    #[error("validation: {0}")]
    Validation(String),
}
