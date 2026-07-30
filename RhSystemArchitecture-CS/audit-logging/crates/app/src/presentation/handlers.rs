//! Axum handlers delegating to use cases held in [`AppState`].

use axum::{
    Json,
    extract::{Query, State},
};
use hrms_platform::error::AppError;

use super::models::{AuditEntryResponse, QueryAuditLogRequest, RecordAuditEntryRequest};
use crate::bootstrap::state::AppState;

pub async fn record_entry(
    State(_state): State<AppState>,
    Json(_body): Json<RecordAuditEntryRequest>,
) -> Result<Json<AuditEntryResponse>, AppError> {
    todo!("map request -> RecordAuditEntryCommand, invoke use case, map result -> response")
}

pub async fn query_log(
    State(_state): State<AppState>,
    Query(_params): Query<QueryAuditLogRequest>,
) -> Result<Json<Vec<AuditEntryResponse>>, AppError> {
    todo!("map params -> QueryAuditLogCommand, invoke use case, map results -> response")
}
