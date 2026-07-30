//! SMTP/email gateway adapter.

use async_trait::async_trait;

use crate::application::ports::ChannelGateway;
use crate::domain::notification::{Notification, NotificationChannel};

pub struct EmailGateway {
    endpoint: String,
}

impl EmailGateway {
    pub fn new(endpoint: String) -> Self {
        Self { endpoint }
    }
}

#[async_trait]
impl ChannelGateway for EmailGateway {
    fn supports(&self, notification: &Notification) -> bool {
        notification.channel == NotificationChannel::Email
    }

    async fn deliver(&self, _notification: &Notification) -> anyhow::Result<()> {
        todo!("POST the message to the configured email provider")
    }
}
