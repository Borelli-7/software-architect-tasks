//! Outbound application ports (implemented by infrastructure adapters).

use async_trait::async_trait;

use crate::domain::employee::Employee;

/// Publishes employee lifecycle events to the message bus.
#[async_trait]
pub trait EmployeeEventPublisher: Send + Sync {
    async fn employee_created(&self, employee: &Employee) -> anyhow::Result<()>;
    async fn employee_terminated(&self, employee: &Employee) -> anyhow::Result<()>;
}

/// Provisions/deprovisions the employee's directory identity (AD/LDAP).
#[async_trait]
pub trait IdentityProvisioner: Send + Sync {
    async fn provision(&self, employee: &Employee) -> anyhow::Result<()>;
    async fn deprovision(&self, employee: &Employee) -> anyhow::Result<()>;
}
