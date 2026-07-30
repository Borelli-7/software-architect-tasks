//! Route table for the audit-logging resources.

use axum::{Router, routing::post};

use super::handlers;
use crate::bootstrap::state::AppState;

pub fn audit_router() -> Router<AppState> {
    Router::new().route(
        "/audit-entries",
        post(handlers::record_entry).get(handlers::query_log),
    )
}
