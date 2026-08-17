//! Public event contract for the Document service.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Kafka topic carrying document lifecycle events.
pub const TOPIC: &str = "document.lifecycle";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DocumentLifecycleEvent {
    DocumentUploaded(DocumentUploaded),
    DocumentGenerated(DocumentGenerated),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentUploaded {
    pub document_id: Uuid,
    pub owner_id: Uuid,
    pub content_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentGenerated {
    pub document_id: Uuid,
    pub owner_id: Uuid,
    pub template_id: Uuid,
}
