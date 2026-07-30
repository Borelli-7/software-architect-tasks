//! Domain ports: persistence, tax computation, and field-level encryption.

use async_trait::async_trait;
use hrms_kernel::Money;

use super::errors::PayrollError;
use super::payroll_run::{PayrollRun, PayrollRunId};
use super::payslip::Payslip;

/// Persistence port for the PayrollRun aggregate.
#[async_trait]
pub trait PayrollRepository: Send + Sync {
    async fn find_by_id(&self, id: PayrollRunId) -> Result<Option<PayrollRun>, PayrollError>;
    async fn save(&self, run: &PayrollRun) -> Result<(), PayrollError>;
}

/// Domain-service port computing tax withholding for a payslip.
#[async_trait]
pub trait TaxEngine: Send + Sync {
    async fn compute_withholding(&self, payslip: &Payslip) -> Result<Money, PayrollError>;
}

/// Field-level encryption port for salary data at rest (AES-256 via HSM/KMS).
pub trait SalaryEncryptor: Send + Sync {
    fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, PayrollError>;
    fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, PayrollError>;
}
