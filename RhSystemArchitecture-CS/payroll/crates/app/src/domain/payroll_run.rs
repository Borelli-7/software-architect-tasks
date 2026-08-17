//! The PayrollRun aggregate and its dual-authorization workflow.

use chrono::{DateTime, NaiveDate, Utc};
use hrms_kernel::Money;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::errors::PayrollError;
use super::payslip::Payslip;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PayrollRunId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EmployeeId(pub Uuid);

/// Accounting period a payroll run covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PayPeriod {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

/// Payroll run lifecycle. Dual authorization gates the move to `Approved`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PayrollRunStatus {
    Draft,
    Calculated,
    PendingFirstApproval,
    PendingSecondApproval,
    Approved,
    Submitted,
    Paid,
    Failed,
}

/// A single sign-off in the dual-control approval workflow.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Approval {
    pub approver_id: Uuid,
    pub approved_at: DateTime<Utc>,
}

/// Aggregate root: a payroll run for a set of employees over one period.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayrollRun {
    pub id: PayrollRunId,
    pub period: PayPeriod,
    pub status: PayrollRunStatus,
    pub payslips: Vec<Payslip>,
    pub approvals: Vec<Approval>,
    pub total: Money,
    pub created_at: DateTime<Utc>,
}

impl PayrollRun {
    /// Records completed gross/net computation (Draft -> Calculated).
    pub fn mark_calculated(&mut self, _total: Money) -> Result<(), PayrollError> {
        todo!("guard status and store computed totals")
    }

    /// Applies one approval, enforcing distinct approvers (dual control).
    pub fn approve(&mut self, _approval: Approval) -> Result<(), PayrollError> {
        todo!("reject duplicate approver and advance the approval state machine")
    }

    /// Transitions to Submitted once fully approved.
    pub fn mark_submitted(&mut self) -> Result<(), PayrollError> {
        todo!("require Approved status before submission")
    }
}
