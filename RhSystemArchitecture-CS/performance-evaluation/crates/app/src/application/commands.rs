//! Command inputs accepted by the evaluation use cases.

use chrono::NaiveDate;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct OpenCampaignCommand {
    pub title: String,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitReviewCommand {
    pub campaign_id: Uuid,
    pub employee_id: Uuid,
    pub reviewer_id: Uuid,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetObjectivesCommand {
    pub employee_id: Uuid,
    pub period: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CloseCampaignCommand {
    pub campaign_id: Uuid,
}
