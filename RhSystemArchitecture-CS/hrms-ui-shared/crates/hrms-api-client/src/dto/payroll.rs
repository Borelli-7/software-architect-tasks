//! Payroll DTOs.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayrollRunDto {
    pub id: Uuid,
    pub period: String,
    pub status: String,
    pub gross_total_minor: i64,
    pub net_total_minor: i64,
    pub currency: String,
    pub initiated_by: Uuid,
    pub approved_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitiatePayrollRunRequest {
    pub period: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovePayrollRunRequest {
    pub approver_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayslipDto {
    pub id: Uuid,
    pub run_id: Uuid,
    pub employee_id: Uuid,
    pub gross_minor: i64,
    pub net_minor: i64,
    pub currency: String,
}
