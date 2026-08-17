//! Axum handlers delegating to use cases held in [`AppState`].

use axum::{
    Json,
    extract::{Path, State},
};
use hrms_platform::error::AppError;
use uuid::Uuid;

use super::models::{
    CompleteTrainingRequest, CreateProgramRequest, EnrollEmployeeRequest, ProgramResponse,
};
use crate::bootstrap::state::AppState;

pub async fn create_program(
    State(_state): State<AppState>,
    Json(_body): Json<CreateProgramRequest>,
) -> Result<Json<ProgramResponse>, AppError> {
    todo!("map request -> CreateProgramCommand, invoke use case, map result -> response")
}

pub async fn enroll_employee(
    State(_state): State<AppState>,
    Path(_program_id): Path<Uuid>,
    Json(_body): Json<EnrollEmployeeRequest>,
) -> Result<Json<ProgramResponse>, AppError> {
    todo!("map request -> EnrollEmployeeCommand, invoke use case, map result -> response")
}

pub async fn complete_training(
    State(_state): State<AppState>,
    Path(_program_id): Path<Uuid>,
    Json(_body): Json<CompleteTrainingRequest>,
) -> Result<Json<ProgramResponse>, AppError> {
    todo!("map request -> CompleteTrainingCommand, invoke use case, map result -> response")
}
