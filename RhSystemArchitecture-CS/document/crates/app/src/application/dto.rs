//! Data-transfer objects returned by document use cases.

use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct DocumentDto {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub filename: String,
    pub content_type: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DocumentContentDto {
    pub content_type: String,
    #[serde(skip)]
    pub bytes: Vec<u8>,
}
