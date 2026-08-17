//! HTTP request/response models for the employee endpoints.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateEmployeeRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub branch_id: Uuid,
    pub department_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEmployeeRequest {
    pub first_name: String,
    pub last_name: String,
    pub department_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct EmployeeResponse {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub status: String,
}
