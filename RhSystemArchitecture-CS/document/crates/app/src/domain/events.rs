//! Domain events emitted by the document aggregate.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::document::DocumentId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentEvent {
    Uploaded { id: DocumentId, owner_id: Uuid },
    Generated { id: DocumentId, owner_id: Uuid, template_id: Uuid },
}
