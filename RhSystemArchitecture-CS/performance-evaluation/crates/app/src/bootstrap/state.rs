//! Axum shared state: the wired use cases exposed to handlers.

use std::sync::Arc;

use crate::application::use_cases::{CloseCampaign, OpenCampaign, SetObjectives, SubmitReview};

#[derive(Clone)]
pub struct AppState {
    pub open_campaign: Arc<OpenCampaign>,
    pub close_campaign: Arc<CloseCampaign>,
    pub submit_review: Arc<SubmitReview>,
    pub set_objectives: Arc<SetObjectives>,
}
