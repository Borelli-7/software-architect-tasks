//! Training program endpoints (`/training-programs`).

use uuid::Uuid;

use crate::ApiClient;
use crate::dto::training::{CreateTrainingProgramRequest, EnrollRequest, TrainingProgramDto};
use crate::error::ApiError;

pub struct TrainingApi<'a>(pub(crate) &'a ApiClient);

impl TrainingApi<'_> {
    pub async fn list(&self) -> Result<Vec<TrainingProgramDto>, ApiError> {
        todo!("GET /training-programs")
    }

    pub async fn get(&self, _id: Uuid) -> Result<TrainingProgramDto, ApiError> {
        todo!("GET /training-programs/{{id}}")
    }

    pub async fn create(&self, _req: CreateTrainingProgramRequest) -> Result<TrainingProgramDto, ApiError> {
        todo!("POST /training-programs")
    }

    pub async fn enroll(&self, _id: Uuid, _req: EnrollRequest) -> Result<(), ApiError> {
        todo!("POST /training-programs/{{id}}/enrollments")
    }
}
