//! Axum handlers delegating to use cases held in [`AppState`].

use axum::{
    Json,
    body::Bytes,
    extract::{Path, State},
};
use hrms_platform::error::AppError;
use uuid::Uuid;

use super::models::{DocumentResponse, GenerateDocumentRequest};
use crate::bootstrap::state::AppState;

pub async fn upload_document(
    State(_state): State<AppState>,
    _body: Bytes,
) -> Result<Json<DocumentResponse>, AppError> {
    todo!("parse metadata + bytes -> UploadDocumentCommand, invoke use case, map result")
}

pub async fn generate_document(
    State(_state): State<AppState>,
    Json(_body): Json<GenerateDocumentRequest>,
) -> Result<Json<DocumentResponse>, AppError> {
    todo!("map request -> GenerateDocumentCommand, invoke use case, map result -> response")
}

pub async fn retrieve_document(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
) -> Result<Bytes, AppError> {
    todo!("map id -> RetrieveDocumentCommand, invoke use case, stream bytes back")
}
