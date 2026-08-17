//! Career path planning endpoints (`/career-paths`).

use uuid::Uuid;

use crate::ApiClient;
use crate::dto::career::{CareerPathDto, DefineCareerPathRequest};
use crate::error::ApiError;

pub struct CareerApi<'a>(pub(crate) &'a ApiClient);

impl CareerApi<'_> {
    pub async fn list(&self) -> Result<Vec<CareerPathDto>, ApiError> {
        todo!("GET /career-paths")
    }

    pub async fn get(&self, _id: Uuid) -> Result<CareerPathDto, ApiError> {
        todo!("GET /career-paths/{{id}}")
    }

    pub async fn define(&self, _req: DefineCareerPathRequest) -> Result<CareerPathDto, ApiError> {
        todo!("POST /career-paths")
    }

    pub async fn complete_milestone(&self, _path_id: Uuid, _milestone_id: Uuid) -> Result<CareerPathDto, ApiError> {
        todo!("POST /career-paths/{{id}}/milestones/{{milestoneId}}/complete")
    }
}
