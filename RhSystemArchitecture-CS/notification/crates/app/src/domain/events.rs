//! Domain events emitted by the notification aggregate.

use serde::{Deserialize, Serialize};

use super::notification::{NotificationChannel, NotificationId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationEvent {
    Dispatched {
        id: NotificationId,
        channel: NotificationChannel,
    },
    Failed {
        id: NotificationId,
        channel: NotificationChannel,
        reason: String,
    },
}
