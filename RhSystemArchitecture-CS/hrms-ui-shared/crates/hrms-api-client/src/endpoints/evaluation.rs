//! Performance evaluation endpoints (`/evaluations`).

use uuid::Uuid;

use crate::ApiClient;
use crate::dto::evaluation::{EvaluationDto, StartEvaluationRequest, SubmitEvaluationRequest};
use crate::error::ApiError;

pub struct EvaluationApi<'a>(pub(crate) &'a ApiClient);

impl EvaluationApi<'_> {
    pub async fn list(&self) -> Result<Vec<EvaluationDto>, ApiError> {
        todo!("GET /evaluations")
    }

    pub async fn get(&self, _id: Uuid) -> Result<EvaluationDto, ApiError> {
        todo!("GET /evaluations/{{id}}")
    }

    pub async fn start(&self, _req: StartEvaluationRequest) -> Result<EvaluationDto, ApiError> {
        todo!("POST /evaluations")
    }

    pub async fn submit(&self, _id: Uuid, _req: SubmitEvaluationRequest) -> Result<EvaluationDto, ApiError> {
        todo!("POST /evaluations/{{id}}/submit")
    }
}
