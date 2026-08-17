//! Command inputs accepted by the iam use cases.

use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserCommand {
    pub username: String,
    pub email: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AssignRoleCommand {
    pub user_id: Uuid,
    pub role: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeactivateUserCommand {
    pub user_id: Uuid,
}
