//! Route table for the evaluation resources.

use axum::{
    Router,
    routing::{get, post},
};

use super::handlers;
use crate::bootstrap::state::AppState;

pub fn evaluation_router() -> Router<AppState> {
    Router::new()
        .route("/campaigns", post(handlers::open_campaign))
        .route("/campaigns/{id}", get(handlers::get_campaign))
        .route("/campaigns/{id}/close", post(handlers::close_campaign))
        .route("/reviews", post(handlers::submit_review))
        .route("/objectives", post(handlers::set_objectives))
}
