//! Outbound application ports (implemented by infrastructure adapters).

use async_trait::async_trait;

use crate::domain::program::TrainingProgram;

/// Publishes training lifecycle events to the message bus.
#[async_trait]
pub trait TrainingEventPublisher: Send + Sync {
    async fn employee_enrolled(&self, program: &TrainingProgram, employee_id: uuid::Uuid) -> anyhow::Result<()>;
    async fn training_completed(&self, program: &TrainingProgram, employee_id: uuid::Uuid) -> anyhow::Result<()>;
}
