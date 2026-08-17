//! Command inputs accepted by the career use cases.

use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct DefineCareerPathCommand {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RunGapAnalysisCommand {
    pub employee_id: Uuid,
    pub path_id: Uuid,
}
