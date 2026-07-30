//! Data-transfer objects returned by iam use cases.

use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct UserDto {
    pub id: Uuid,
    pub username: String,
    pub status: String,
    pub roles: Vec<String>,
}
