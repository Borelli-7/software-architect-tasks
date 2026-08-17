//! Kafka adapter publishing career lifecycle events to the message bus.

use std::sync::Arc;

use async_trait::async_trait;
use hrms_messaging::EventPublisher;

use crate::application::ports::CareerEventPublisher;
use crate::domain::career_path::CareerPath;
use crate::domain::competency::GapAnalysis;

pub struct KafkaCareerEventPublisher {
    publisher: Arc<dyn EventPublisher>,
    topic: String,
}

impl KafkaCareerEventPublisher {
    pub fn new(publisher: Arc<dyn EventPublisher>, topic: String) -> Self {
        Self { publisher, topic }
    }
}

#[async_trait]
impl CareerEventPublisher for KafkaCareerEventPublisher {
    async fn career_path_defined(&self, _path: &CareerPath) -> anyhow::Result<()> {
        todo!("map to CareerPathDefined contract and publish to topic")
    }

    async fn gap_analysis_completed(&self, _analysis: &GapAnalysis) -> anyhow::Result<()> {
        todo!("map to GapAnalysisCompleted contract and publish to topic")
    }
}
