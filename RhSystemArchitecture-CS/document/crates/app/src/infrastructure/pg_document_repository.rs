//! PostgreSQL adapter for the Document aggregate.

use async_trait::async_trait;
use hrms_persistence::DbPool;

use crate::domain::document::{Document, DocumentId};
use crate::domain::errors::DocumentError;
use crate::domain::ports::DocumentRepository;

pub struct PgDocumentRepository {
    pool: DbPool,
}

impl PgDocumentRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DocumentRepository for PgDocumentRepository {
    async fn find_by_id(&self, _id: DocumentId) -> Result<Option<Document>, DocumentError> {
        todo!("SELECT document metadata by id")
    }

    async fn save(&self, _document: &Document) -> Result<(), DocumentError> {
        todo!("UPSERT document metadata row")
    }
}
