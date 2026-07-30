//! The Review entity: a single supervisor/self assessment within a campaign.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::campaign::CampaignId;
use super::errors::EvaluationError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ReviewId(pub Uuid);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewStatus {
    Draft,
    Submitted,
    Validated,
}

/// A scored competency line item on a review.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetencyScore {
    pub competency: String,
    pub score: u8,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub id: ReviewId,
    pub campaign_id: CampaignId,
    pub employee_id: Uuid,
    pub reviewer_id: Uuid,
    pub status: ReviewStatus,
    pub scores: Vec<CompetencyScore>,
}

impl Review {
    /// Submits a draft review after validating score completeness.
    pub fn submit(&mut self) -> Result<(), EvaluationError> {
        todo!("validate all required competencies scored, then mark Submitted")
    }
}
