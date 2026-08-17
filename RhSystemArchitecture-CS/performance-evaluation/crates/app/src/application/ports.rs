//! Outbound application ports (implemented by infrastructure adapters).

use async_trait::async_trait;

use crate::domain::campaign::EvaluationCampaign;
use crate::domain::review::Review;

/// Publishes evaluation lifecycle events to the message bus.
#[async_trait]
pub trait EvaluationEventPublisher: Send + Sync {
    async fn campaign_opened(&self, campaign: &EvaluationCampaign) -> anyhow::Result<()>;
    async fn evaluation_completed(&self, review: &Review, score: f32) -> anyhow::Result<()>;
}
