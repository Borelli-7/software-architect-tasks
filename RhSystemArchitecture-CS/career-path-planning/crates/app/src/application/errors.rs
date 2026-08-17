//! Application-level error type aggregating domain and infrastructure failures.

use thiserror::Error;

use crate::domain::errors::CareerError;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error(transparent)]
    Domain(#[from] CareerError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
