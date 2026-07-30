//! Public event contract for the Career Path Planning service.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Kafka topic carrying career lifecycle events.
pub const TOPIC: &str = "career.lifecycle";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CareerLifecycleEvent {
    CareerPathDefined(CareerPathDefined),
    GapAnalysisCompleted(GapAnalysisCompleted),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CareerPathDefined {
    pub path_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GapAnalysisCompleted {
    pub employee_id: Uuid,
    pub path_id: Uuid,
    pub gap_count: u32,
}
