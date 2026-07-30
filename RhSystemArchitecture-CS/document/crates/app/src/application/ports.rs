//! Outbound application ports (implemented by infrastructure adapters).

use async_trait::async_trait;

use crate::domain::document::{Document, DocumentMetadata};

/// Object storage for document bytes (S3, Azure Blob, GCS).
#[async_trait]
pub trait BlobStore: Send + Sync {
    async fn put(&self, key: &str, bytes: &[u8]) -> anyhow::Result<()>;
    async fn get(&self, key: &str) -> anyhow::Result<Vec<u8>>;
}

/// Renders documents from templates (e.g. contract PDFs).
#[async_trait]
pub trait DocumentGenerator: Send + Sync {
    async fn generate(
        &self,
        template_id: uuid::Uuid,
        params: &[(String, String)],
    ) -> anyhow::Result<(Vec<u8>, DocumentMetadata)>;
}

/// Publishes document lifecycle events to the message bus.
#[async_trait]
pub trait DocumentEventPublisher: Send + Sync {
    async fn uploaded(&self, document: &Document) -> anyhow::Result<()>;
    async fn generated(&self, document: &Document, template_id: uuid::Uuid) -> anyhow::Result<()>;
}
