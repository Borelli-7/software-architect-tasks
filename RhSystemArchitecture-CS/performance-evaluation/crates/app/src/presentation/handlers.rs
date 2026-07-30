//! Axum handlers delegating to use cases held in [`AppState`].

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use hrms_platform::error::AppError;
use uuid::Uuid;

use super::models::{
    CampaignResponse, OpenCampaignRequest, ReviewResponse, SetObjectivesRequest, SubmitReviewRequest,
};
use crate::bootstrap::state::AppState;

pub async fn open_campaign(
    State(_state): State<AppState>,
    Json(_body): Json<OpenCampaignRequest>,
) -> Result<Json<CampaignResponse>, AppError> {
    todo!("map request -> OpenCampaignCommand, invoke use case, map result -> response")
}

pub async fn get_campaign(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
) -> Result<Json<CampaignResponse>, AppError> {
    todo!("invoke query, map result -> response")
}

pub async fn close_campaign(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
) -> Result<Json<CampaignResponse>, AppError> {
    todo!("map path -> CloseCampaignCommand, invoke use case, map result -> response")
}

pub async fn submit_review(
    State(_state): State<AppState>,
    Json(_body): Json<SubmitReviewRequest>,
) -> Result<Json<ReviewResponse>, AppError> {
    todo!("map request -> SubmitReviewCommand, invoke use case, map result -> response")
}

pub async fn set_objectives(
    State(_state): State<AppState>,
    Json(_body): Json<SetObjectivesRequest>,
) -> Result<StatusCode, AppError> {
    todo!("map request -> SetObjectivesCommand, invoke use case, return 201")
}
