//! Axum shared state: the wired use cases exposed to handlers.

use std::sync::Arc;

use crate::application::use_cases::{CompleteTraining, CreateProgram, EnrollEmployee};

#[derive(Clone)]
pub struct AppState {
    pub create_program: Arc<CreateProgram>,
    pub enroll_employee: Arc<EnrollEmployee>,
    pub complete_training: Arc<CompleteTraining>,
}
