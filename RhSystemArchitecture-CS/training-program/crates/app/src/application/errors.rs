//! Application-level error type aggregating domain and infrastructure failures.

use thiserror::Error;

use crate::domain::errors::TrainingError;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error(transparent)]
    Domain(#[from] TrainingError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
