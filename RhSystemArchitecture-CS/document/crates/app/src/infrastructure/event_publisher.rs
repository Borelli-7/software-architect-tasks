//! Kafka adapter publishing document lifecycle events to the message bus.

use std::sync::Arc;

use async_trait::async_trait;
use hrms_messaging::EventPublisher;
use uuid::Uuid;

use crate::application::ports::DocumentEventPublisher;
use crate::domain::document::Document;

pub struct KafkaDocumentEventPublisher {
    publisher: Arc<dyn EventPublisher>,
    topic: String,
}

impl KafkaDocumentEventPublisher {
    pub fn new(publisher: Arc<dyn EventPublisher>, topic: String) -> Self {
        Self { publisher, topic }
    }
}

#[async_trait]
impl DocumentEventPublisher for KafkaDocumentEventPublisher {
    async fn uploaded(&self, _document: &Document) -> anyhow::Result<()> {
        todo!("map to DocumentUploaded contract and publish to topic")
    }

    async fn generated(&self, _document: &Document, _template_id: Uuid) -> anyhow::Result<()> {
        todo!("map to DocumentGenerated contract and publish to topic")
    }
}
