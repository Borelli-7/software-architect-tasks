//! PostgreSQL adapter for the TrainingProgram aggregate.

use async_trait::async_trait;
use hrms_persistence::DbPool;

use crate::domain::errors::TrainingError;
use crate::domain::ports::TrainingRepository;
use crate::domain::program::{ProgramId, TrainingProgram};

pub struct PgTrainingRepository {
    pool: DbPool,
}

impl PgTrainingRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TrainingRepository for PgTrainingRepository {
    async fn find_by_id(&self, _id: ProgramId) -> Result<Option<TrainingProgram>, TrainingError> {
        todo!("SELECT program + enrollments")
    }

    async fn save(&self, _program: &TrainingProgram) -> Result<(), TrainingError> {
        todo!("UPSERT program + enrollments")
    }
}
