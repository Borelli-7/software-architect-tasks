//! Performance evaluation DTOs.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationDto {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub reviewer_id: Uuid,
    pub cycle: String,
    pub status: String,
    pub overall_score: Option<f32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartEvaluationRequest {
    pub employee_id: Uuid,
    pub reviewer_id: Uuid,
    pub cycle: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitEvaluationRequest {
    pub overall_score: f32,
    pub summary: String,
}
