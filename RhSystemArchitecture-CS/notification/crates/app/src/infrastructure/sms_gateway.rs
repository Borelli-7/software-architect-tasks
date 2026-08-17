//! SMS gateway adapter.

use async_trait::async_trait;

use crate::application::ports::ChannelGateway;
use crate::domain::notification::{Notification, NotificationChannel};

pub struct SmsGateway {
    endpoint: String,
}

impl SmsGateway {
    pub fn new(endpoint: String) -> Self {
        Self { endpoint }
    }
}

#[async_trait]
impl ChannelGateway for SmsGateway {
    fn supports(&self, notification: &Notification) -> bool {
        notification.channel == NotificationChannel::Sms
    }

    async fn deliver(&self, _notification: &Notification) -> anyhow::Result<()> {
        todo!("POST the message to the configured SMS provider")
    }
}
