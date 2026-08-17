//! Application-level error type aggregating domain and infrastructure failures.

use thiserror::Error;

use crate::domain::errors::AuditError;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error(transparent)]
    Domain(#[from] AuditError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
