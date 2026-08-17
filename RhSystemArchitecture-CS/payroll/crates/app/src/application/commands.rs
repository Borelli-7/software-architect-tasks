//! Command inputs accepted by the payroll use cases.

use chrono::NaiveDate;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct RunPayrollCommand {
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub department_id: Option<Uuid>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApprovePayrollCommand {
    pub run_id: Uuid,
    pub approver_id: Uuid,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitPaymentBatchCommand {
    pub run_id: Uuid,
}
