//! Template-based document generator adapter (e.g. PDF rendering service).

use async_trait::async_trait;
use uuid::Uuid;

use crate::application::ports::DocumentGenerator;
use crate::domain::document::DocumentMetadata;

pub struct HttpDocumentGenerator {
    endpoint: String,
}

impl HttpDocumentGenerator {
    pub fn new(endpoint: String) -> Self {
        Self { endpoint }
    }
}

#[async_trait]
impl DocumentGenerator for HttpDocumentGenerator {
    async fn generate(
        &self,
        _template_id: Uuid,
        _params: &[(String, String)],
    ) -> anyhow::Result<(Vec<u8>, DocumentMetadata)> {
        todo!("call the rendering service and return bytes + metadata")
    }
}
