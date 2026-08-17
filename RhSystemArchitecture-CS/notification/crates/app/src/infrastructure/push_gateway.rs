//! Push-notification gateway adapter.

use async_trait::async_trait;

use crate::application::ports::ChannelGateway;
use crate::domain::notification::{Notification, NotificationChannel};

pub struct PushGateway {
    endpoint: String,
}

impl PushGateway {
    pub fn new(endpoint: String) -> Self {
        Self { endpoint }
    }
}

#[async_trait]
impl ChannelGateway for PushGateway {
    fn supports(&self, notification: &Notification) -> bool {
        notification.channel == NotificationChannel::Push
    }

    async fn deliver(&self, _notification: &Notification) -> anyhow::Result<()> {
        todo!("POST the message to the configured push provider (FCM/APNs)")
    }
}
