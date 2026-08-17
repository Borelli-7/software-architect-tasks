//! Domain events emitted by the career aggregates.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::career_path::CareerPathId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CareerEvent {
    CareerPathDefined { id: CareerPathId },
    GapAnalysisCompleted { employee_id: Uuid, path_id: CareerPathId },
}
