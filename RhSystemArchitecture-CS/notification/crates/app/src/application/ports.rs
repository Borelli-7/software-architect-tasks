//! Outbound application ports (implemented by infrastructure adapters).

use async_trait::async_trait;

use crate::domain::notification::Notification;

/// Delivers a notification through a concrete channel gateway.
#[async_trait]
pub trait ChannelGateway: Send + Sync {
    /// Whether this gateway handles the notification's channel.
    fn supports(&self, notification: &Notification) -> bool;
    async fn deliver(&self, notification: &Notification) -> anyhow::Result<()>;
}

/// Publishes notification lifecycle events to the message bus.
#[async_trait]
pub trait NotificationEventPublisher: Send + Sync {
    async fn dispatched(&self, notification: &Notification) -> anyhow::Result<()>;
    async fn failed(&self, notification: &Notification, reason: &str) -> anyhow::Result<()>;
}
