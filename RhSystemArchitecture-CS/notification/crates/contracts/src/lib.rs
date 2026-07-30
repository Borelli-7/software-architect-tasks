//! Public event contract for the Notification service.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Kafka topic carrying notification lifecycle events.
pub const TOPIC: &str = "notification.lifecycle";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NotificationLifecycleEvent {
    NotificationDispatched(NotificationDispatched),
    NotificationFailed(NotificationFailed),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationDispatched {
    pub notification_id: Uuid,
    pub recipient_id: Uuid,
    pub channel: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationFailed {
    pub notification_id: Uuid,
    pub recipient_id: Uuid,
    pub channel: String,
    pub reason: String,
}
