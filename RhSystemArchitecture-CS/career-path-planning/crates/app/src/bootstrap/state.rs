//! Axum shared state: the wired use cases exposed to handlers.

use std::sync::Arc;

use crate::application::use_cases::{DefineCareerPath, RunGapAnalysis};

#[derive(Clone)]
pub struct AppState {
    pub define_career_path: Arc<DefineCareerPath>,
    pub run_gap_analysis: Arc<RunGapAnalysis>,
}
