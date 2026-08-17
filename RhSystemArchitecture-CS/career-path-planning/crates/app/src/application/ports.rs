//! Outbound application ports (implemented by infrastructure adapters).

use async_trait::async_trait;

use crate::domain::career_path::CareerPath;
use crate::domain::competency::GapAnalysis;

/// Publishes career lifecycle events to the message bus.
#[async_trait]
pub trait CareerEventPublisher: Send + Sync {
    async fn career_path_defined(&self, path: &CareerPath) -> anyhow::Result<()>;
    async fn gap_analysis_completed(&self, analysis: &GapAnalysis) -> anyhow::Result<()>;
}
