//! Route table for the document resources.

use axum::{
    Router,
    routing::{get, post},
};

use super::handlers;
use crate::bootstrap::state::AppState;

pub fn document_router() -> Router<AppState> {
    Router::new()
        .route("/documents", post(handlers::upload_document))
        .route("/documents/generate", post(handlers::generate_document))
        .route("/documents/{id}", get(handlers::retrieve_document))
}
