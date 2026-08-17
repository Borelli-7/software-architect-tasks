//! Read adapter for employee competencies (sourced from evaluation data).

use async_trait::async_trait;
use hrms_persistence::DbPool;
use uuid::Uuid;

use crate::domain::competency::EmployeeCompetency;
use crate::domain::errors::CareerError;
use crate::domain::ports::CompetencyRepository;

pub struct PgCompetencyRepository {
    pool: DbPool,
}

impl PgCompetencyRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CompetencyRepository for PgCompetencyRepository {
    async fn list_for_employee(&self, _employee_id: Uuid) -> Result<Vec<EmployeeCompetency>, CareerError> {
        todo!("SELECT assessed competencies for the employee")
    }
}
