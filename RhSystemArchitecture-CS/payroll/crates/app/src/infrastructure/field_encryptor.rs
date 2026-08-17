//! Field-level salary encryption backed by an HSM/KMS-managed data key.

use crate::domain::errors::PayrollError;
use crate::domain::ports::SalaryEncryptor;

pub struct HsmSalaryEncryptor {
    key_id: String,
}

impl HsmSalaryEncryptor {
    pub fn new(key_id: String) -> Self {
        Self { key_id }
    }
}

impl SalaryEncryptor for HsmSalaryEncryptor {
    fn encrypt(&self, _plaintext: &[u8]) -> Result<Vec<u8>, PayrollError> {
        todo!("AES-256-GCM encrypt using the HSM data key identified by key_id")
    }

    fn decrypt(&self, _ciphertext: &[u8]) -> Result<Vec<u8>, PayrollError> {
        todo!("authenticate and decrypt using the HSM data key identified by key_id")
    }
}
