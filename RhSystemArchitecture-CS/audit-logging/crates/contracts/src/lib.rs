//! Public event contract for the Audit Logging service.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Kafka topic carrying audit lifecycle events.
pub const TOPIC: &str = "audit.lifecycle";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AuditLifecycleEvent {
    AuditEntryRecorded(AuditEntryRecorded),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntryRecorded {
    pub entry_id: Uuid,
    pub actor_id: Uuid,
    pub action: String,
    pub resource: String,
}
