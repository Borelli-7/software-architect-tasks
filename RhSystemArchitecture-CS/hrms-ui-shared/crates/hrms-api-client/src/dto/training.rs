//! Training program DTOs.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingProgramDto {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub starts_on: NaiveDate,
    pub ends_on: NaiveDate,
    pub capacity: u32,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTrainingProgramRequest {
    pub title: String,
    pub description: String,
    pub starts_on: NaiveDate,
    pub ends_on: NaiveDate,
    pub capacity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrollRequest {
    pub employee_id: Uuid,
}
