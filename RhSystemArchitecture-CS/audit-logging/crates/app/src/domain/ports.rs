//! Domain ports: append-only persistence and query for audit entries.

use async_trait::async_trait;
use uuid::Uuid;

use super::entry::{AuditEntry, AuditEntryId};
use super::errors::AuditError;

/// A read-side filter used when querying the audit log.
#[derive(Debug, Clone, Default)]
pub struct AuditQuery {
    pub actor_id: Option<Uuid>,
    pub action: Option<String>,
    pub resource: Option<String>,
    pub limit: u32,
}

#[async_trait]
pub trait AuditRepository: Send + Sync {
    /// Appends a new entry. Implementations must never update existing rows.
    async fn append(&self, entry: &AuditEntry) -> Result<(), AuditError>;
    async fn find_by_id(&self, id: AuditEntryId) -> Result<Option<AuditEntry>, AuditError>;
    async fn query(&self, query: &AuditQuery) -> Result<Vec<AuditEntry>, AuditError>;
}
