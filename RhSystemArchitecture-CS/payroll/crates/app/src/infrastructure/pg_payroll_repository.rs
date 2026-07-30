//! PostgreSQL adapter for the PayrollRun aggregate.

use async_trait::async_trait;
use hrms_persistence::DbPool;

use crate::domain::errors::PayrollError;
use crate::domain::payroll_run::{PayrollRun, PayrollRunId};
use crate::domain::ports::PayrollRepository;

pub struct PgPayrollRepository {
    pool: DbPool,
}

impl PgPayrollRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PayrollRepository for PgPayrollRepository {
    async fn find_by_id(&self, _id: PayrollRunId) -> Result<Option<PayrollRun>, PayrollError> {
        todo!("SELECT run + payslips + approvals, hydrate aggregate")
    }

    async fn save(&self, _run: &PayrollRun) -> Result<(), PayrollError> {
        todo!("UPSERT aggregate within a transaction")
    }
}
