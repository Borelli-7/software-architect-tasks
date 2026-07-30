//! PostgreSQL adapter for the CareerPath aggregate.

use async_trait::async_trait;
use hrms_persistence::DbPool;

use crate::domain::career_path::{CareerPath, CareerPathId};
use crate::domain::errors::CareerError;
use crate::domain::ports::CareerPathRepository;

pub struct PgCareerPathRepository {
    pool: DbPool,
}

impl PgCareerPathRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CareerPathRepository for PgCareerPathRepository {
    async fn find_by_id(&self, _id: CareerPathId) -> Result<Option<CareerPath>, CareerError> {
        todo!("SELECT career path + steps")
    }

    async fn save(&self, _path: &CareerPath) -> Result<(), CareerError> {
        todo!("UPSERT career path + steps")
    }
}
