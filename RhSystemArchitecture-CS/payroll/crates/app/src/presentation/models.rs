//! Request/response wire models for the payroll HTTP API.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct RunPayrollRequest {
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub department_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct ApprovePayrollRequest {
    pub approver_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct PayrollRunResponse {
    pub id: Uuid,
    pub status: String,
    pub total_amount: String,
    pub currency: String,
    pub payslip_count: usize,
}
