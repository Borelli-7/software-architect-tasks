//! Audit logging DTOs.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntryDto {
    pub id: Uuid,
    pub actor_id: Uuid,
    pub action: String,
    pub resource: String,
    pub metadata: serde_json::Value,
    pub recorded_at: DateTime<Utc>,
}

/// Query filter for the audit log listing endpoint.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuditQuery {
    pub actor_id: Option<Uuid>,
    pub action: Option<String>,
    pub resource: Option<String>,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
}
