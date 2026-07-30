//! Event-driven messaging abstractions (pub/sub + transactional outbox).
//!
//! The publisher/consumer traits are object-safe so services can hold
//! `Arc<dyn EventPublisher>`. A concrete Kafka adapter is added in a follow-up;
//! [`InMemoryEventPublisher`] enables local wiring and tests.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum MessagingError {
    #[error("serialization error: {0}")]
    Serialization(String),
    #[error("transport error: {0}")]
    Transport(String),
}

/// Marker for domain events that can be enveloped and published.
pub trait DomainEvent: Serialize + Send + Sync {
    fn event_type(&self) -> &'static str;
    fn aggregate_id(&self) -> Uuid;
}

/// Transport envelope wrapping a serialized domain event with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope<T> {
    pub id: Uuid,
    pub event_type: String,
    pub aggregate_id: Uuid,
    pub occurred_at: DateTime<Utc>,
    pub payload: T,
}

impl<T> EventEnvelope<T> {
    pub fn new(event_type: impl Into<String>, aggregate_id: Uuid, payload: T) -> Self {
        Self {
            id: Uuid::new_v4(),
            event_type: event_type.into(),
            aggregate_id,
            occurred_at: Utc::now(),
            payload,
        }
    }
}

/// Publishes serialized events to a topic. Object-safe by design.
#[async_trait]
pub trait EventPublisher: Send + Sync {
    async fn publish(
        &self,
        topic: &str,
        key: &str,
        payload: serde_json::Value,
    ) -> Result<(), MessagingError>;
}

/// Handles inbound event payloads for a subscribed topic.
#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn handle(&self, topic: &str, payload: &[u8]) -> Result<(), MessagingError>;
}

/// Long-running consumer that dispatches to registered handlers.
#[async_trait]
pub trait EventConsumer: Send + Sync {
    async fn start(&self) -> Result<(), MessagingError>;
}

/// Transactional outbox: persist-then-publish for at-least-once delivery.
#[async_trait]
pub trait Outbox: Send + Sync {
    async fn enqueue(
        &self,
        topic: &str,
        key: &str,
        payload: serde_json::Value,
    ) -> Result<(), MessagingError>;
}

/// No-op publisher for local wiring and tests.
#[derive(Debug, Default, Clone)]
pub struct InMemoryEventPublisher;

#[async_trait]
impl EventPublisher for InMemoryEventPublisher {
    async fn publish(
        &self,
        _topic: &str,
        _key: &str,
        _payload: serde_json::Value,
    ) -> Result<(), MessagingError> {
        Ok(())
    }
}
