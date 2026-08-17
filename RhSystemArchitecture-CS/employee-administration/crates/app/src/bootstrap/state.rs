//! Axum shared state: the wired use cases exposed to handlers.

use std::sync::Arc;

use crate::application::use_cases::{CreateEmployee, TerminateEmployee, UpdateEmployee};

#[derive(Clone)]
pub struct AppState {
    pub create_employee: Arc<CreateEmployee>,
    pub update_employee: Arc<UpdateEmployee>,
    pub terminate_employee: Arc<TerminateEmployee>,
}
