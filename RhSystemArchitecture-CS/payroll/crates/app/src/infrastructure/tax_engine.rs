//! Rule-based tax withholding engine (progressive brackets, allowances).

use async_trait::async_trait;
use hrms_kernel::Money;

use crate::domain::errors::PayrollError;
use crate::domain::payslip::Payslip;
use crate::domain::ports::TaxEngine;

#[derive(Default)]
pub struct RuleBasedTaxEngine;

impl RuleBasedTaxEngine {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl TaxEngine for RuleBasedTaxEngine {
    async fn compute_withholding(&self, _payslip: &Payslip) -> Result<Money, PayrollError> {
        todo!("apply configured tax brackets and statutory deductions")
    }
}
