//! Request/response wire models for the career HTTP API.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct DefineCareerPathRequest {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct RunGapAnalysisRequest {
    pub employee_id: Uuid,
    pub path_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct CareerPathResponse {
    pub id: Uuid,
    pub name: String,
    pub step_count: usize,
}

#[derive(Debug, Serialize)]
pub struct GapAnalysisResponse {
    pub employee_id: Uuid,
    pub path_id: Uuid,
    pub gap_count: usize,
}
