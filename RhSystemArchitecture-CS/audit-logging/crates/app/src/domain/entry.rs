//! The AuditEntry aggregate: an immutable, append-only audit record.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::errors::AuditError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AuditEntryId(pub Uuid);

/// Aggregate root: one immutable audit record. Never updated once written.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub id: AuditEntryId,
    pub actor_id: Uuid,
    pub action: String,
    pub resource: String,
    pub metadata: serde_json::Value,
    pub occurred_at: DateTime<Utc>,
}

impl AuditEntry {
    /// Validates a new entry before it is appended to the log.
    pub fn validate(&self) -> Result<(), AuditError> {
        todo!("enforce required fields (actor, action, resource)")
    }
}
