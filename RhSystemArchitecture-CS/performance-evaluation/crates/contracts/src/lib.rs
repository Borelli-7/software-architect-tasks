//! Public event contract for the Performance Evaluation service.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Kafka topic carrying evaluation lifecycle events.
pub const TOPIC: &str = "evaluation.lifecycle";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EvaluationLifecycleEvent {
    CampaignOpened(CampaignOpened),
    EvaluationCompleted(EvaluationCompleted),
    ObjectivesSet(ObjectivesSet),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignOpened {
    pub campaign_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationCompleted {
    pub campaign_id: Uuid,
    pub employee_id: Uuid,
    pub overall_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectivesSet {
    pub employee_id: Uuid,
    pub period: String,
}
