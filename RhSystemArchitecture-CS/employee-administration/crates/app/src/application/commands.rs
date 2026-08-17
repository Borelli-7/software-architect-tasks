//! Command inputs accepted by the application use cases.

use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateEmployeeCommand {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub branch_id: Uuid,
    pub department_id: Uuid,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateEmployeeCommand {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub department_id: Uuid,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TerminateEmployeeCommand {
    pub id: Uuid,
}
