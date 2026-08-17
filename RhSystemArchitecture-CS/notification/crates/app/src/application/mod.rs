//! Application layer: notification use cases and outbound ports.

pub mod commands;
pub mod dto;
pub mod errors;
pub mod ports;
pub mod use_cases;

use async_trait::async_trait;

use errors::ApplicationError;

/// Generic command handler contract implemented by each use case.
#[async_trait]
pub trait CommandHandler<C>: Send + Sync {
    type Output;
    async fn handle(&self, command: C) -> Result<Self::Output, ApplicationError>;
}
