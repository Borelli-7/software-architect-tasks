//! Command inputs accepted by the document use cases.

use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct UploadDocumentCommand {
    pub owner_id: Uuid,
    pub filename: String,
    pub content_type: String,
    #[serde(skip)]
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GenerateDocumentCommand {
    pub owner_id: Uuid,
    pub template_id: Uuid,
    pub params: Vec<(String, String)>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RetrieveDocumentCommand {
    pub document_id: Uuid,
}
