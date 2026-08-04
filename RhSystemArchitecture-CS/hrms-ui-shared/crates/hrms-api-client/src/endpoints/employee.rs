//! Employee administration endpoints (`/employees`).

use uuid::Uuid;

use crate::ApiClient;
use crate::dto::employee::{CreateEmployeeRequest, EmployeeDto, UpdateEmployeeRequest};
use crate::error::ApiError;

pub struct EmployeeApi<'a>(pub(crate) &'a ApiClient);

impl EmployeeApi<'_> {
    pub async fn list(&self) -> Result<Vec<EmployeeDto>, ApiError> {
        todo!("GET {}", self.0.url("/employees"))
    }

    pub async fn get(&self, _id: Uuid) -> Result<EmployeeDto, ApiError> {
        todo!("GET /employees/{{id}}")
    }

    pub async fn create(&self, _req: CreateEmployeeRequest) -> Result<EmployeeDto, ApiError> {
        todo!("POST /employees")
    }

    pub async fn update(&self, _id: Uuid, _req: UpdateEmployeeRequest) -> Result<EmployeeDto, ApiError> {
        todo!("PATCH /employees/{{id}}")
    }
}
