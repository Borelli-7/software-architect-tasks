//! Inbound domain ports (driven adapters implement these in infrastructure).

use async_trait::async_trait;
use uuid::Uuid;

use super::employee::{Employee, EmployeeId};
use super::errors::EmployeeError;

/// Persistence port for the Employee aggregate.
#[async_trait]
pub trait EmployeeRepository: Send + Sync {
    async fn find_by_id(&self, id: EmployeeId) -> Result<Option<Employee>, EmployeeError>;
    async fn save(&self, employee: &Employee) -> Result<(), EmployeeError>;
    async fn delete(&self, id: EmployeeId) -> Result<(), EmployeeError>;
    async fn list_by_department(
        &self,
        department_id: Uuid,
    ) -> Result<Vec<Employee>, EmployeeError>;
}
