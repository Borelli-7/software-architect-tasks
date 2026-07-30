//! Route table for the payroll-run resource.

use axum::{
    Router,
    routing::{get, post},
};

use super::handlers;
use crate::bootstrap::state::AppState;

pub fn payroll_router() -> Router<AppState> {
    Router::new()
        .route(
            "/payroll-runs",
            post(handlers::run_payroll),
        )
        .route("/payroll-runs/{id}", get(handlers::get_payroll_run))
        .route(
            "/payroll-runs/{id}/approvals",
            post(handlers::approve_payroll),
        )
        .route(
            "/payroll-runs/{id}/submit",
            post(handlers::submit_payment_batch),
        )
}
