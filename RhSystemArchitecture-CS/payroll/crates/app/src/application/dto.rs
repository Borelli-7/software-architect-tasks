//! Data-transfer objects returned by payroll use cases.

use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct PayrollRunDto {
    pub id: Uuid,
    pub status: String,
    pub total_amount: String,
    pub currency: String,
    pub payslip_count: usize,
}
