//! Outbound application ports (implemented by infrastructure adapters).

use async_trait::async_trait;

use crate::domain::entry::AuditEntry;

/// Publishes audit lifecycle events to the message bus.
#[async_trait]
pub trait AuditEventPublisher: Send + Sync {
    async fn recorded(&self, entry: &AuditEntry) -> anyhow::Result<()>;
}
