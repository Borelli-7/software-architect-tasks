//! Domain events emitted by the training aggregate.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::program::ProgramId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrainingEvent {
    EmployeeEnrolled { program_id: ProgramId, employee_id: Uuid },
    TrainingCompleted { program_id: ProgramId, employee_id: Uuid },
}
