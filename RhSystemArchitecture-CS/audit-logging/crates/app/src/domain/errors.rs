//! Domain errors for the audit-logging bounded context.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuditError {
    #[error("audit entry not found")]
    NotFound,
    #[error("audit log is append-only")]
    AppendOnlyViolation,
    #[error("validation error: {0}")]
    Validation(String),
}
