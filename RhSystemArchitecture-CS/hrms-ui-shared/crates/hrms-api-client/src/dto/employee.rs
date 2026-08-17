//! Employee administration DTOs.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeDto {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub department: String,
    pub job_title: String,
    pub hired_on: NaiveDate,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEmployeeRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub department: String,
    pub job_title: String,
    pub hired_on: NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEmployeeRequest {
    pub department: Option<String>,
    pub job_title: Option<String>,
    pub status: Option<String>,
}
