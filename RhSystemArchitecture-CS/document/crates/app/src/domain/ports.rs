//! Domain ports: persistence for the Document aggregate.

use async_trait::async_trait;

use super::document::{Document, DocumentId};
use super::errors::DocumentError;

#[async_trait]
pub trait DocumentRepository: Send + Sync {
    async fn find_by_id(&self, id: DocumentId) -> Result<Option<Document>, DocumentError>;
    async fn save(&self, document: &Document) -> Result<(), DocumentError>;
}
