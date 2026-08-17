//! Route table for the notification resources.

use axum::{Router, routing::post};

use super::handlers;
use crate::bootstrap::state::AppState;

pub fn notification_router() -> Router<AppState> {
    Router::new()
        .route("/notifications", post(handlers::send_notification))
        .route("/templates/render", post(handlers::render_template))
}
