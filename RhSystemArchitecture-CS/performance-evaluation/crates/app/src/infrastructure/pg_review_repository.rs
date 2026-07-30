//! PostgreSQL adapter for reviews.

use async_trait::async_trait;
use hrms_persistence::DbPool;

use crate::domain::campaign::CampaignId;
use crate::domain::errors::EvaluationError;
use crate::domain::ports::ReviewRepository;
use crate::domain::review::{Review, ReviewId};

pub struct PgReviewRepository {
    pool: DbPool,
}

impl PgReviewRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ReviewRepository for PgReviewRepository {
    async fn find_by_id(&self, _id: ReviewId) -> Result<Option<Review>, EvaluationError> {
        todo!("SELECT review by id")
    }

    async fn save(&self, _review: &Review) -> Result<(), EvaluationError> {
        todo!("UPSERT review + scores")
    }

    async fn list_by_campaign(&self, _id: CampaignId) -> Result<Vec<Review>, EvaluationError> {
        todo!("SELECT reviews by campaign")
    }
}
