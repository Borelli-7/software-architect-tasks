//! Public event contract for the Training Program service.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Kafka topic carrying training lifecycle events.
pub const TOPIC: &str = "training.lifecycle";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TrainingLifecycleEvent {
    EmployeeEnrolled(EmployeeEnrolled),
    TrainingCompleted(TrainingCompleted),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeEnrolled {
    pub program_id: Uuid,
    pub employee_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingCompleted {
    pub program_id: Uuid,
    pub employee_id: Uuid,
    pub certificate_id: Option<Uuid>,
}
