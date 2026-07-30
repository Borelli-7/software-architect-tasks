//! PostgreSQL append-only adapter for the AuditEntry aggregate.

use async_trait::async_trait;
use hrms_persistence::DbPool;

use crate::domain::entry::{AuditEntry, AuditEntryId};
use crate::domain::errors::AuditError;
use crate::domain::ports::{AuditQuery, AuditRepository};

pub struct PgAuditRepository {
    pool: DbPool,
}

impl PgAuditRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuditRepository for PgAuditRepository {
    async fn append(&self, _entry: &AuditEntry) -> Result<(), AuditError> {
        todo!("INSERT audit row; table has no UPDATE/DELETE grants")
    }

    async fn find_by_id(&self, _id: AuditEntryId) -> Result<Option<AuditEntry>, AuditError> {
        todo!("SELECT audit entry by id")
    }

    async fn query(&self, _query: &AuditQuery) -> Result<Vec<AuditEntry>, AuditError> {
        todo!("SELECT audit entries matching the filter, ordered by occurred_at")
    }
}
