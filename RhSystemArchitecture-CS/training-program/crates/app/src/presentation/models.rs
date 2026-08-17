//! Request/response wire models for the training HTTP API.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateProgramRequest {
    pub title: String,
    pub capacity: u32,
    pub external_provider: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EnrollEmployeeRequest {
    pub employee_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct CompleteTrainingRequest {
    pub employee_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct ProgramResponse {
    pub id: Uuid,
    pub title: String,
    pub capacity: u32,
    pub enrolled_count: usize,
}
