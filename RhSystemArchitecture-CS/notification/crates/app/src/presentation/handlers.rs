//! Axum handlers delegating to use cases held in [`AppState`].

use axum::{Json, extract::State};
use hrms_platform::error::AppError;

use super::models::{
    NotificationResponse, RenderTemplateRequest, RenderedTemplateResponse, SendNotificationRequest,
};
use crate::bootstrap::state::AppState;

pub async fn send_notification(
    State(_state): State<AppState>,
    Json(_body): Json<SendNotificationRequest>,
) -> Result<Json<NotificationResponse>, AppError> {
    todo!("map request -> SendNotificationCommand, invoke use case, map result -> response")
}

pub async fn render_template(
    State(_state): State<AppState>,
    Json(_body): Json<RenderTemplateRequest>,
) -> Result<Json<RenderedTemplateResponse>, AppError> {
    todo!("map request -> RenderTemplateCommand, invoke use case, map result -> response")
}
