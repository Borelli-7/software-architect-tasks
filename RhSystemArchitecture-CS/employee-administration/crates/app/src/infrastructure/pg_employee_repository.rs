//! PostgreSQL adapter for [`EmployeeRepository`] using sqlx.

use async_trait::async_trait;
use hrms_persistence::DbPool;
use uuid::Uuid;

use crate::domain::employee::{Employee, EmployeeId};
use crate::domain::errors::EmployeeError;
use crate::domain::ports::EmployeeRepository;

pub struct PgEmployeeRepository {
    pool: DbPool,
}

impl PgEmployeeRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EmployeeRepository for PgEmployeeRepository {
    async fn find_by_id(&self, _id: EmployeeId) -> Result<Option<Employee>, EmployeeError> {
        let _pool = &self.pool;
        todo!("SELECT by id via sqlx and map row -> Employee")
    }

    async fn save(&self, _employee: &Employee) -> Result<(), EmployeeError> {
        todo!("INSERT ... ON CONFLICT DO UPDATE via sqlx")
    }

    async fn delete(&self, _id: EmployeeId) -> Result<(), EmployeeError> {
        todo!("soft-delete row via sqlx")
    }

    async fn list_by_department(
        &self,
        _department_id: Uuid,
    ) -> Result<Vec<Employee>, EmployeeError> {
        todo!("SELECT list by department via sqlx")
    }
}
