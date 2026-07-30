//! Kafka adapter publishing audit lifecycle events to the message bus.

use std::sync::Arc;

use async_trait::async_trait;
use hrms_messaging::EventPublisher;

use crate::application::ports::AuditEventPublisher;
use crate::domain::entry::AuditEntry;

pub struct KafkaAuditEventPublisher {
    publisher: Arc<dyn EventPublisher>,
    topic: String,
}

impl KafkaAuditEventPublisher {
    pub fn new(publisher: Arc<dyn EventPublisher>, topic: String) -> Self {
        Self { publisher, topic }
    }
}

#[async_trait]
impl AuditEventPublisher for KafkaAuditEventPublisher {
    async fn recorded(&self, _entry: &AuditEntry) -> anyhow::Result<()> {
        todo!("map to AuditEntryRecorded contract and publish to topic")
    }
}
