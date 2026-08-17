//! Domain errors for the payroll bounded context.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum PayrollError {
    #[error("payroll run not found")]
    NotFound,
    #[error("invalid status transition")]
    InvalidTransition,
    #[error("approver already signed off")]
    DuplicateApprover,
    #[error("dual authorization incomplete")]
    ApprovalIncomplete,
    #[error("calculation error: {0}")]
    Calculation(String),
    #[error("encryption error: {0}")]
    Encryption(String),
}
