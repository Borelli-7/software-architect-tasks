//! Adapter bridging the application's [`EmployeeEventPublisher`] port onto the
//! shared messaging [`EventPublisher`] (Kafka in production).

use std::sync::Arc;

use async_trait::async_trait;
use hrms_messaging::EventPublisher;

use crate::application::ports::EmployeeEventPublisher;
use crate::domain::employee::Employee;

pub struct KafkaEmployeeEventPublisher {
    publisher: Arc<dyn EventPublisher>,
    topic: String,
}

impl KafkaEmployeeEventPublisher {
    pub fn new(publisher: Arc<dyn EventPublisher>, topic: impl Into<String>) -> Self {
        Self { publisher, topic: topic.into() }
    }
}

#[async_trait]
impl EmployeeEventPublisher for KafkaEmployeeEventPublisher {
    async fn employee_created(&self, _employee: &Employee) -> anyhow::Result<()> {
        let _ = (&self.publisher, &self.topic);
        todo!("build EmployeeCreated envelope and publish to topic")
    }

    async fn employee_terminated(&self, _employee: &Employee) -> anyhow::Result<()> {
        todo!("build EmployeeTerminated envelope and publish to topic")
    }
}
