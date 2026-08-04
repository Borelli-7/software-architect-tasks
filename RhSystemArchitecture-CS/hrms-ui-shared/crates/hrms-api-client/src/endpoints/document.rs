//! Document endpoints (`/documents`).

use uuid::Uuid;

use crate::ApiClient;
use crate::dto::document::{DocumentDto, GenerateDocumentRequest};
use crate::error::ApiError;

pub struct DocumentApi<'a>(pub(crate) &'a ApiClient);

impl DocumentApi<'_> {
    pub async fn upload(&self, _file_name: &str, _bytes: Vec<u8>) -> Result<DocumentDto, ApiError> {
        todo!("POST /documents (raw bytes body)")
    }

    pub async fn generate(&self, _req: GenerateDocumentRequest) -> Result<DocumentDto, ApiError> {
        todo!("POST /documents/generate")
    }

    pub async fn retrieve(&self, _id: Uuid) -> Result<Vec<u8>, ApiError> {
        todo!("GET /documents/{{id}}")
    }
}
