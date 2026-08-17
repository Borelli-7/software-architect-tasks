//! Kafka adapter publishing evaluation lifecycle events to the message bus.

use std::sync::Arc;

use async_trait::async_trait;
use hrms_messaging::EventPublisher;

use crate::application::ports::EvaluationEventPublisher;
use crate::domain::campaign::EvaluationCampaign;
use crate::domain::review::Review;

pub struct KafkaEvaluationEventPublisher {
    publisher: Arc<dyn EventPublisher>,
    topic: String,
}

impl KafkaEvaluationEventPublisher {
    pub fn new(publisher: Arc<dyn EventPublisher>, topic: String) -> Self {
        Self { publisher, topic }
    }
}

#[async_trait]
impl EvaluationEventPublisher for KafkaEvaluationEventPublisher {
    async fn campaign_opened(&self, _campaign: &EvaluationCampaign) -> anyhow::Result<()> {
        todo!("map to CampaignOpened contract and publish to topic")
    }

    async fn evaluation_completed(&self, _review: &Review, _score: f32) -> anyhow::Result<()> {
        todo!("map to EvaluationCompleted contract and publish to topic")
    }
}
