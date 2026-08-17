//! Command inputs accepted by the audit-logging use cases.

use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct RecordAuditEntryCommand {
    pub actor_id: Uuid,
    pub action: String,
    pub resource: String,
    #[serde(default)]
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize)]
pub struct QueryAuditLogCommand {
    pub actor_id: Option<Uuid>,
    pub action: Option<String>,
    pub resource: Option<String>,
    #[serde(default = "default_limit")]
    pub limit: u32,
}

fn default_limit() -> u32 {
    100
}
