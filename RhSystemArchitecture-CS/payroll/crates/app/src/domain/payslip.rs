//! Payslip value objects composing a payroll run.

use hrms_kernel::Money;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Category of a payslip line item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PayComponentKind {
    BaseSalary,
    Bonus,
    Commission,
    Overtime,
    Deduction,
    Tax,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayComponent {
    pub kind: PayComponentKind,
    pub label: String,
    pub amount: Money,
}

/// Per-employee payslip within a payroll run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payslip {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub components: Vec<PayComponent>,
    pub gross: Money,
    pub net: Money,
}
