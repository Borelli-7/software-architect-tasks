//! HTTP request/response models for the audit-logging resources.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct RecordAuditEntryRequest {
    pub actor_id: Uuid,
    pub action: String,
    pub resource: String,
    #[serde(default)]
    pub metadata: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct QueryAuditLogRequest {
    pub actor_id: Option<Uuid>,
    pub action: Option<String>,
    pub resource: Option<String>,
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct AuditEntryResponse {
    pub id: Uuid,
    pub actor_id: Uuid,
    pub action: String,
    pub resource: String,
    pub occurred_at: DateTime<Utc>,
}
