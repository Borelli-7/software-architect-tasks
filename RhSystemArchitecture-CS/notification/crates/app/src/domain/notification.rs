//! The Notification aggregate and its delivery channels.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::errors::NotificationError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NotificationId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationChannel {
    Email,
    Sms,
    Push,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeliveryStatus {
    Pending,
    Dispatched,
    Failed,
}

/// Aggregate root: a single message queued for delivery on one channel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: NotificationId,
    pub recipient_id: Uuid,
    pub channel: NotificationChannel,
    pub subject: Option<String>,
    pub body: String,
    pub status: DeliveryStatus,
    pub created_at: DateTime<Utc>,
}

impl Notification {
    /// Marks the notification dispatched after the gateway accepts it.
    pub fn mark_dispatched(&mut self) -> Result<(), NotificationError> {
        todo!("guard status and transition Pending -> Dispatched")
    }

    /// Marks the notification failed, capturing the gateway reason.
    pub fn mark_failed(&mut self, _reason: &str) -> Result<(), NotificationError> {
        todo!("guard status and transition Pending -> Failed")
    }
}
