//! Route table for the training resources.

use axum::{Router, routing::post};

use super::handlers;
use crate::bootstrap::state::AppState;

pub fn training_router() -> Router<AppState> {
    Router::new()
        .route("/programs", post(handlers::create_program))
        .route("/programs/{id}/enrollments", post(handlers::enroll_employee))
        .route("/programs/{id}/completions", post(handlers::complete_training))
}
