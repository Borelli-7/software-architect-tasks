//! Outbound application ports (implemented by infrastructure adapters).

use async_trait::async_trait;

use crate::domain::payroll_run::PayrollRun;

/// Publishes payroll lifecycle events to the message bus.
#[async_trait]
pub trait PayrollEventPublisher: Send + Sync {
    async fn payroll_approved(&self, run: &PayrollRun) -> anyhow::Result<()>;
    async fn payroll_paid(&self, run: &PayrollRun) -> anyhow::Result<()>;
}

/// Acknowledgement returned by the core-banking system for a payment batch.
#[derive(Debug, Clone)]
pub struct PaymentAck {
    pub reference: String,
}

/// Saga coordinating the payroll -> core-banking payment transaction with a
/// compensating action on failure.
#[async_trait]
pub trait PaymentSaga: Send + Sync {
    async fn submit_batch(&self, run: &PayrollRun) -> anyhow::Result<PaymentAck>;
    async fn compensate(&self, run: &PayrollRun) -> anyhow::Result<()>;
}
