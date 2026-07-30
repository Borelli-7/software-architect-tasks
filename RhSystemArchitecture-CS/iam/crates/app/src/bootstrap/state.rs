//! Axum shared state: the wired use cases exposed to handlers.

use std::sync::Arc;

use crate::application::use_cases::{AssignRole, CreateUser, DeactivateUser};

#[derive(Clone)]
pub struct AppState {
    pub create_user: Arc<CreateUser>,
    pub assign_role: Arc<AssignRole>,
    pub deactivate_user: Arc<DeactivateUser>,
}
