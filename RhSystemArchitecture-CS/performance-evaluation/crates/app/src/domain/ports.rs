//! Domain ports: persistence and the analytics scoring service.

use async_trait::async_trait;

use super::campaign::{CampaignId, EvaluationCampaign};
use super::errors::EvaluationError;
use super::objective::Objective;
use super::review::{Review, ReviewId};

/// Persistence port for the EvaluationCampaign aggregate.
#[async_trait]
pub trait CampaignRepository: Send + Sync {
    async fn find_by_id(&self, id: CampaignId) -> Result<Option<EvaluationCampaign>, EvaluationError>;
    async fn save(&self, campaign: &EvaluationCampaign) -> Result<(), EvaluationError>;
}

/// Persistence port for individual reviews.
#[async_trait]
pub trait ReviewRepository: Send + Sync {
    async fn find_by_id(&self, id: ReviewId) -> Result<Option<Review>, EvaluationError>;
    async fn save(&self, review: &Review) -> Result<(), EvaluationError>;
    async fn list_by_campaign(&self, id: CampaignId) -> Result<Vec<Review>, EvaluationError>;
}

/// Persistence port for objectives.
#[async_trait]
pub trait ObjectiveRepository: Send + Sync {
    async fn save(&self, objective: &Objective) -> Result<(), EvaluationError>;
    async fn list_by_employee(&self, employee_id: uuid::Uuid) -> Result<Vec<Objective>, EvaluationError>;
}

/// Domain-service port computing aggregate scores and trends.
#[async_trait]
pub trait AnalyticsEngine: Send + Sync {
    async fn overall_score(&self, review: &Review) -> Result<f32, EvaluationError>;
}
