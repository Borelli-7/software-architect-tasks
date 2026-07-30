//! Domain events emitted by the audit-logging aggregate.

use serde::{Deserialize, Serialize};

use super::entry::AuditEntryId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditEvent {
    Recorded { id: AuditEntryId },
}
