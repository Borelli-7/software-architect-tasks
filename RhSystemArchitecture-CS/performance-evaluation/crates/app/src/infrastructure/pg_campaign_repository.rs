//! PostgreSQL adapter for the EvaluationCampaign aggregate.

use async_trait::async_trait;
use hrms_persistence::DbPool;

use crate::domain::campaign::{CampaignId, EvaluationCampaign};
use crate::domain::errors::EvaluationError;
use crate::domain::ports::CampaignRepository;

pub struct PgCampaignRepository {
    pool: DbPool,
}

impl PgCampaignRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CampaignRepository for PgCampaignRepository {
    async fn find_by_id(&self, _id: CampaignId) -> Result<Option<EvaluationCampaign>, EvaluationError> {
        todo!("SELECT campaign by id")
    }

    async fn save(&self, _campaign: &EvaluationCampaign) -> Result<(), EvaluationError> {
        todo!("UPSERT campaign")
    }
}
