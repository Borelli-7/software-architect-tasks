//! Request/response wire models for the evaluation HTTP API.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct OpenCampaignRequest {
    pub title: String,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
}

#[derive(Debug, Deserialize)]
pub struct SubmitReviewRequest {
    pub campaign_id: Uuid,
    pub employee_id: Uuid,
    pub reviewer_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct SetObjectivesRequest {
    pub employee_id: Uuid,
    pub period: String,
}

#[derive(Debug, Serialize)]
pub struct CampaignResponse {
    pub id: Uuid,
    pub title: String,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct ReviewResponse {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub status: String,
    pub overall_score: f32,
}
