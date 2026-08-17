//! Data-transfer objects returned by career use cases.

use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct CareerPathDto {
    pub id: Uuid,
    pub name: String,
    pub step_count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct GapAnalysisDto {
    pub employee_id: Uuid,
    pub path_id: Uuid,
    pub gap_count: usize,
}
