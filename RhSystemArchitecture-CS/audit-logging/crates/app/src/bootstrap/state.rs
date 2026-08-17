//! Axum shared state: the wired use cases exposed to handlers.

use std::sync::Arc;

use crate::application::use_cases::{QueryAuditLog, RecordAuditEntry};

#[derive(Clone)]
pub struct AppState {
    pub record_entry: Arc<RecordAuditEntry>,
    pub query_log: Arc<QueryAuditLog>,
}
