//! Application-level error type aggregating domain and infrastructure failures.

use thiserror::Error;

use crate::domain::errors::PayrollError;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error(transparent)]
    Domain(#[from] PayrollError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
