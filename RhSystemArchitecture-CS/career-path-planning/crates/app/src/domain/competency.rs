//! Competency value objects and the gap-analysis result.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequiredCompetency {
    pub name: String,
    pub required_level: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeCompetency {
    pub name: String,
    pub current_level: u8,
}

/// A single skill shortfall found during gap analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetencyGap {
    pub name: String,
    pub required_level: u8,
    pub current_level: u8,
}

/// Result of comparing an employee's competencies against a career step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GapAnalysis {
    pub employee_id: Uuid,
    pub path_id: Uuid,
    pub gaps: Vec<CompetencyGap>,
}
