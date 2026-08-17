//! Career path planning DTOs.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CareerPathDto {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub current_role: String,
    pub target_role: String,
    pub status: String,
    pub milestones: Vec<MilestoneDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MilestoneDto {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefineCareerPathRequest {
    pub employee_id: Uuid,
    pub current_role: String,
    pub target_role: String,
}
