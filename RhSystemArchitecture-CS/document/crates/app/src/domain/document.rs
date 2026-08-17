//! The Document aggregate and its stored metadata.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::errors::DocumentError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DocumentId(pub Uuid);

/// Metadata describing a stored blob without holding its bytes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub filename: String,
    pub content_type: String,
    pub size_bytes: u64,
    pub storage_key: String,
}

/// Aggregate root: an owned document with a blob-store reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: DocumentId,
    pub owner_id: Uuid,
    pub metadata: DocumentMetadata,
    pub created_at: DateTime<Utc>,
}

impl Document {
    /// Validates that the metadata describes an accepted document type.
    pub fn validate(&self) -> Result<(), DocumentError> {
        todo!("enforce allowed content types and size limits")
    }
}
