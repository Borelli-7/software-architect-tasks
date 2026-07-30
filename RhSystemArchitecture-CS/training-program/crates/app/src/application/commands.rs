//! Command inputs accepted by the training use cases.

use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateProgramCommand {
    pub title: String,
    pub capacity: u32,
    pub external_provider: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnrollEmployeeCommand {
    pub program_id: Uuid,
    pub employee_id: Uuid,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CompleteTrainingCommand {
    pub program_id: Uuid,
    pub employee_id: Uuid,
}
