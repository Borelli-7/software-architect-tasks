//! The Objective entity: a quarterly goal tracked against an employee.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ObjectiveId(pub Uuid);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObjectiveStatus {
    Set,
    InProgress,
    Achieved,
    Missed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Objective {
    pub id: ObjectiveId,
    pub employee_id: Uuid,
    pub description: String,
    pub period: String,
    pub progress_pct: u8,
    pub status: ObjectiveStatus,
}
