//! Kafka adapter publishing payroll lifecycle events to the message bus.

use std::sync::Arc;

use async_trait::async_trait;
use hrms_messaging::EventPublisher;

use crate::application::ports::PayrollEventPublisher;
use crate::domain::payroll_run::PayrollRun;

pub struct KafkaPayrollEventPublisher {
    publisher: Arc<dyn EventPublisher>,
    topic: String,
}

impl KafkaPayrollEventPublisher {
    pub fn new(publisher: Arc<dyn EventPublisher>, topic: String) -> Self {
        Self { publisher, topic }
    }
}

#[async_trait]
impl PayrollEventPublisher for KafkaPayrollEventPublisher {
    async fn payroll_approved(&self, _run: &PayrollRun) -> anyhow::Result<()> {
        todo!("map aggregate to PayrollApproved contract and publish to topic")
    }

    async fn payroll_paid(&self, _run: &PayrollRun) -> anyhow::Result<()> {
        todo!("map aggregate to PayrollPaid contract and publish to topic")
    }
}
