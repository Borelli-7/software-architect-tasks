//! Kafka adapter publishing notification lifecycle events to the message bus.

use std::sync::Arc;

use async_trait::async_trait;
use hrms_messaging::EventPublisher;

use crate::application::ports::NotificationEventPublisher;
use crate::domain::notification::Notification;

pub struct KafkaNotificationEventPublisher {
    publisher: Arc<dyn EventPublisher>,
    topic: String,
}

impl KafkaNotificationEventPublisher {
    pub fn new(publisher: Arc<dyn EventPublisher>, topic: String) -> Self {
        Self { publisher, topic }
    }
}

#[async_trait]
impl NotificationEventPublisher for KafkaNotificationEventPublisher {
    async fn dispatched(&self, _notification: &Notification) -> anyhow::Result<()> {
        todo!("map to NotificationDispatched contract and publish to topic")
    }

    async fn failed(&self, _notification: &Notification, _reason: &str) -> anyhow::Result<()> {
        todo!("map to NotificationFailed contract and publish to topic")
    }
}
