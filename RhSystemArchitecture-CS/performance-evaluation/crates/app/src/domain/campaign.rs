//! The EvaluationCampaign aggregate and its review-cycle state machine.

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::errors::EvaluationError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CampaignId(pub Uuid);

/// Campaign lifecycle (mirrors the evaluation workflow state machine).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CampaignStatus {
    Planned,
    Open,
    InProgress,
    PendingReview,
    Closed,
}

/// Aggregate root: an evaluation campaign spanning a review period.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationCampaign {
    pub id: CampaignId,
    pub title: String,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub status: CampaignStatus,
    pub created_at: DateTime<Utc>,
}

impl EvaluationCampaign {
    /// Opens the campaign for supervisor reviews (Planned -> Open).
    pub fn open(&mut self) -> Result<(), EvaluationError> {
        todo!("guard status and transition to Open")
    }

    /// Closes the campaign once all reviews are validated.
    pub fn close(&mut self) -> Result<(), EvaluationError> {
        todo!("require PendingReview and transition to Closed")
    }
}
