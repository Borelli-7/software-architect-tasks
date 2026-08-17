//! Domain events emitted by the evaluation aggregates.

use serde::{Deserialize, Serialize};

use super::campaign::CampaignId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvaluationEvent {
    CampaignOpened { id: CampaignId },
    ReviewCompleted { id: CampaignId, employee_id: uuid::Uuid },
    CampaignClosed { id: CampaignId },
}
