//! Data-transfer objects returned by audit-logging use cases.

use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct AuditEntryDto {
    pub id: Uuid,
    pub actor_id: Uuid,
    pub action: String,
    pub resource: String,
    pub occurred_at: DateTime<Utc>,
}
