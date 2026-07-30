//! Domain errors for the notification bounded context.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum NotificationError {
    #[error("notification not found")]
    NotFound,
    #[error("template not found")]
    TemplateNotFound,
    #[error("unsupported channel")]
    UnsupportedChannel,
    #[error("invalid status transition")]
    InvalidTransition,
    #[error("validation error: {0}")]
    Validation(String),
}
