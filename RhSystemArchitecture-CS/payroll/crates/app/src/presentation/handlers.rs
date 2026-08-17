//! Axum handlers. Each maps HTTP <-> application command and delegates to a use
//! case held in [`AppState`].

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use hrms_platform::error::AppError;
use uuid::Uuid;

use super::models::{ApprovePayrollRequest, PayrollRunResponse, RunPayrollRequest};
use crate::bootstrap::state::AppState;

pub async fn run_payroll(
    State(_state): State<AppState>,
    Json(_body): Json<RunPayrollRequest>,
) -> Result<Json<PayrollRunResponse>, AppError> {
    todo!("map request -> RunPayrollCommand, invoke use case, map result -> response")
}

pub async fn approve_payroll(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
    Json(_body): Json<ApprovePayrollRequest>,
) -> Result<Json<PayrollRunResponse>, AppError> {
    todo!("map request -> ApprovePayrollCommand, invoke use case, map result -> response")
}

pub async fn submit_payment_batch(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    todo!("map path -> SubmitPaymentBatchCommand, invoke use case, return 202")
}

pub async fn get_payroll_run(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
) -> Result<Json<PayrollRunResponse>, AppError> {
    todo!("invoke query, map result -> response")
}
