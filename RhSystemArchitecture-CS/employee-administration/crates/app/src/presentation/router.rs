//! Route table for the employee resource.

use axum::{
    Router,
    routing::{get, post},
};

use super::handlers;
use crate::bootstrap::state::AppState;

pub fn employee_router() -> Router<AppState> {
    Router::new()
        .route(
            "/employees",
            post(handlers::create_employee).get(handlers::list_employees),
        )
        .route(
            "/employees/{id}",
            get(handlers::get_employee).put(handlers::update_employee),
        )
}
