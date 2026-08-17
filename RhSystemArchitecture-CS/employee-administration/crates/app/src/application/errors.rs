//! Application-level error type aggregating domain and infrastructure failures.

use thiserror::Error;

use crate::domain::errors::EmployeeError;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error(transparent)]
    Domain(#[from] EmployeeError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
