//! PostgreSQL adapter for objectives.

use async_trait::async_trait;
use hrms_persistence::DbPool;
use uuid::Uuid;

use crate::domain::errors::EvaluationError;
use crate::domain::objective::Objective;
use crate::domain::ports::ObjectiveRepository;

pub struct PgObjectiveRepository {
    pool: DbPool,
}

impl PgObjectiveRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ObjectiveRepository for PgObjectiveRepository {
    async fn save(&self, _objective: &Objective) -> Result<(), EvaluationError> {
        todo!("UPSERT objective")
    }

    async fn list_by_employee(&self, _employee_id: Uuid) -> Result<Vec<Objective>, EvaluationError> {
        todo!("SELECT objectives by employee")
    }
}
