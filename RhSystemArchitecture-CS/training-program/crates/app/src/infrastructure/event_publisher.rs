//! Kafka adapter publishing training lifecycle events to the message bus.

use std::sync::Arc;

use async_trait::async_trait;
use hrms_messaging::EventPublisher;
use uuid::Uuid;

use crate::application::ports::TrainingEventPublisher;
use crate::domain::program::TrainingProgram;

pub struct KafkaTrainingEventPublisher {
    publisher: Arc<dyn EventPublisher>,
    topic: String,
}

impl KafkaTrainingEventPublisher {
    pub fn new(publisher: Arc<dyn EventPublisher>, topic: String) -> Self {
        Self { publisher, topic }
    }
}

#[async_trait]
impl TrainingEventPublisher for KafkaTrainingEventPublisher {
    async fn employee_enrolled(&self, _program: &TrainingProgram, _employee_id: Uuid) -> anyhow::Result<()> {
        todo!("map to EmployeeEnrolled contract and publish to topic")
    }

    async fn training_completed(&self, _program: &TrainingProgram, _employee_id: Uuid) -> anyhow::Result<()> {
        todo!("map to TrainingCompleted contract and publish to topic")
    }
}
