//! Axum handlers delegating to use cases held in [`AppState`].

use axum::{
    Json,
    extract::{Path, State},
};
use hrms_platform::error::AppError;
use uuid::Uuid;

use super::models::{AssignRoleRequest, CreateUserRequest, UserResponse};
use crate::bootstrap::state::AppState;

pub async fn create_user(
    State(_state): State<AppState>,
    Json(_body): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    todo!("map request -> CreateUserCommand, invoke use case, map result -> response")
}

pub async fn get_user(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
) -> Result<Json<UserResponse>, AppError> {
    todo!("load user by id, map result -> response")
}

pub async fn assign_role(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
    Json(_body): Json<AssignRoleRequest>,
) -> Result<Json<UserResponse>, AppError> {
    todo!("map request -> AssignRoleCommand, invoke use case, map result -> response")
}

pub async fn deactivate_user(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
) -> Result<Json<UserResponse>, AppError> {
    todo!("map request -> DeactivateUserCommand, invoke use case, map result -> response")
}
