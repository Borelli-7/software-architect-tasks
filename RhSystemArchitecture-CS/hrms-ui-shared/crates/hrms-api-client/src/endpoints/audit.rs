//! Audit logging endpoints (`/audit-entries`).

use crate::ApiClient;
use crate::dto::audit::{AuditEntryDto, AuditQuery};
use crate::error::ApiError;

pub struct AuditApi<'a>(pub(crate) &'a ApiClient);

impl AuditApi<'_> {
    pub async fn query(&self, _filter: AuditQuery) -> Result<Vec<AuditEntryDto>, ApiError> {
        todo!("GET /audit-entries with query filter")
    }
}
