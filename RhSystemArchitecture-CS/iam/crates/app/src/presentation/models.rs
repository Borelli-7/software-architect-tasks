//! HTTP request/response models for the iam resources.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct AssignRoleRequest {
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub status: String,
    pub roles: Vec<String>,
}
