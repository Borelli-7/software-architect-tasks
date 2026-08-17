//! Axum handlers. Each maps HTTP <-> application command/query and delegates to
//! a use case held in [`AppState`].

use axum::{
    Json,
    extract::{Path, State},
};
use hrms_platform::error::AppError;
use uuid::Uuid;

use super::models::{CreateEmployeeRequest, EmployeeResponse, UpdateEmployeeRequest};
use crate::bootstrap::state::AppState;

pub async fn create_employee(
    State(_state): State<AppState>,
    Json(_body): Json<CreateEmployeeRequest>,
) -> Result<Json<EmployeeResponse>, AppError> {
    todo!("map request -> CreateEmployeeCommand, invoke use case, map result -> response")
}

pub async fn update_employee(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
    Json(_body): Json<UpdateEmployeeRequest>,
) -> Result<Json<EmployeeResponse>, AppError> {
    todo!("map request -> UpdateEmployeeCommand, invoke use case, map result -> response")
}

pub async fn get_employee(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
) -> Result<Json<EmployeeResponse>, AppError> {
    todo!("invoke query, map result -> response")
}

pub async fn list_employees(
    State(_state): State<AppState>,
) -> Result<Json<Vec<EmployeeResponse>>, AppError> {
    todo!("invoke list query, map results -> responses")
}
