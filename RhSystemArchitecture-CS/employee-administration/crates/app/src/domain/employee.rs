//! The Employee aggregate and its value objects.

use chrono::{DateTime, Utc};
use hrms_kernel::Email;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::errors::EmployeeError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EmployeeId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BranchId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DepartmentId(pub Uuid);

/// Employee lifecycle states (mirrors the HRMS state machine).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmploymentStatus {
    Onboarding,
    Active,
    Suspended,
    Terminated,
}

/// Aggregate root for an employee record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employee {
    pub id: EmployeeId,
    pub first_name: String,
    pub last_name: String,
    pub email: Email,
    pub branch_id: BranchId,
    pub department_id: DepartmentId,
    pub status: EmploymentStatus,
    pub hired_at: DateTime<Utc>,
    pub terminated_at: Option<DateTime<Utc>>,
}

impl Employee {
    /// Transitions the aggregate to `Terminated`, enforcing lifecycle invariants.
    pub fn terminate(&mut self, _at: DateTime<Utc>) -> Result<(), EmployeeError> {
        todo!("guard current status and set terminated_at + status")
    }

    /// Reassigns the employee to a different department.
    pub fn reassign_department(&mut self, _department_id: DepartmentId) -> Result<(), EmployeeError> {
        todo!("validate transition and update department")
    }
}
