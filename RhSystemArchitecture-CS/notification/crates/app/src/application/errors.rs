//! Application-level error type aggregating domain and infrastructure failures.

use thiserror::Error;

use crate::domain::errors::NotificationError;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error(transparent)]
    Domain(#[from] NotificationError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
