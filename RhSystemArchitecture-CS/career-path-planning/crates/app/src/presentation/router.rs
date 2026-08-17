//! Route table for the career resources.

use axum::{Router, routing::post};

use super::handlers;
use crate::bootstrap::state::AppState;

pub fn career_router() -> Router<AppState> {
    Router::new()
        .route("/career-paths", post(handlers::define_career_path))
        .route("/gap-analyses", post(handlers::run_gap_analysis))
}
