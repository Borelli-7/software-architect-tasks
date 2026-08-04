//! IAM endpoints (`/users`).

use uuid::Uuid;

use crate::ApiClient;
use crate::dto::iam::{AssignRoleRequest, CreateUserRequest, UserDto};
use crate::error::ApiError;

pub struct IamApi<'a>(pub(crate) &'a ApiClient);

impl IamApi<'_> {
    pub async fn list(&self) -> Result<Vec<UserDto>, ApiError> {
        todo!("GET /users")
    }

    pub async fn get(&self, _id: Uuid) -> Result<UserDto, ApiError> {
        todo!("GET /users/{{id}}")
    }

    pub async fn create(&self, _req: CreateUserRequest) -> Result<UserDto, ApiError> {
        todo!("POST /users")
    }

    pub async fn assign_role(&self, _id: Uuid, _req: AssignRoleRequest) -> Result<UserDto, ApiError> {
        todo!("POST /users/{{id}}/roles")
    }

    pub async fn deactivate(&self, _id: Uuid) -> Result<UserDto, ApiError> {
        todo!("POST /users/{{id}}/deactivate")
    }
}
