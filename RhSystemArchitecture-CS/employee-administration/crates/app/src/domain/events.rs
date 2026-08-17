//! Domain events emitted by the employee aggregate.

use serde::{Deserialize, Serialize};

use super::employee::EmployeeId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmployeeEvent {
    Created { id: EmployeeId },
    Updated { id: EmployeeId },
    Terminated { id: EmployeeId },
}
