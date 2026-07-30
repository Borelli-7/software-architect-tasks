//! Axum handlers delegating to use cases held in [`AppState`].

use axum::{Json, extract::State};
use hrms_platform::error::AppError;

use super::models::{
    CareerPathResponse, DefineCareerPathRequest, GapAnalysisResponse, RunGapAnalysisRequest,
};
use crate::bootstrap::state::AppState;

pub async fn define_career_path(
    State(_state): State<AppState>,
    Json(_body): Json<DefineCareerPathRequest>,
) -> Result<Json<CareerPathResponse>, AppError> {
    todo!("map request -> DefineCareerPathCommand, invoke use case, map result -> response")
}

pub async fn run_gap_analysis(
    State(_state): State<AppState>,
    Json(_body): Json<RunGapAnalysisRequest>,
) -> Result<Json<GapAnalysisResponse>, AppError> {
    todo!("map request -> RunGapAnalysisCommand, invoke use case, map result -> response")
}
