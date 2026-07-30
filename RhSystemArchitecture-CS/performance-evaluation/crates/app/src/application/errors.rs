//! Application-level error type aggregating domain and infrastructure failures.

use thiserror::Error;

use crate::domain::errors::EvaluationError;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error(transparent)]
    Domain(#[from] EvaluationError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
