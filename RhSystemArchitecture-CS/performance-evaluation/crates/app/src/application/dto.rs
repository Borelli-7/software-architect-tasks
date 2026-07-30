//! Data-transfer objects returned by evaluation use cases.

use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct CampaignDto {
    pub id: Uuid,
    pub title: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReviewDto {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub status: String,
    pub overall_score: f32,
}
