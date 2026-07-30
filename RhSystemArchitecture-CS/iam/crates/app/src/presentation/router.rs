//! Route table for the iam resources.

use axum::{
    Router,
    routing::{get, post},
};

use super::handlers;
use crate::bootstrap::state::AppState;

pub fn iam_router() -> Router<AppState> {
    Router::new()
        .route("/users", post(handlers::create_user))
        .route("/users/{id}", get(handlers::get_user))
        .route("/users/{id}/roles", post(handlers::assign_role))
        .route("/users/{id}/deactivate", post(handlers::deactivate_user))
}
