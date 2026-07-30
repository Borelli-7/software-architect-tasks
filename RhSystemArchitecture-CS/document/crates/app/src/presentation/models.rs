//! HTTP request/response models for the document resources.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct GenerateDocumentRequest {
    pub owner_id: Uuid,
    pub template_id: Uuid,
    pub params: Vec<(String, String)>,
}

#[derive(Debug, Serialize)]
pub struct DocumentResponse {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub filename: String,
    pub content_type: String,
}
