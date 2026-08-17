//! Document DTOs.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentDto {
    pub id: Uuid,
    pub file_name: String,
    pub content_type: String,
    pub size_bytes: u64,
    pub owner_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateDocumentRequest {
    pub template_key: String,
    pub owner_id: Uuid,
    pub variables: serde_json::Value,
}
