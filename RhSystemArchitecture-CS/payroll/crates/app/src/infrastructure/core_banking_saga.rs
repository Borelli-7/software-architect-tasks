//! Saga adapter driving payment settlement through the core-banking system.

use async_trait::async_trait;

use crate::application::ports::{PaymentAck, PaymentSaga};
use crate::domain::payroll_run::PayrollRun;

pub struct CoreBankingSaga {
    endpoint: String,
}

impl CoreBankingSaga {
    pub fn new(endpoint: String) -> Self {
        Self { endpoint }
    }
}

#[async_trait]
impl PaymentSaga for CoreBankingSaga {
    async fn submit_batch(&self, _run: &PayrollRun) -> anyhow::Result<PaymentAck> {
        todo!("POST payment batch to core banking endpoint, await acknowledgement")
    }

    async fn compensate(&self, _run: &PayrollRun) -> anyhow::Result<()> {
        todo!("issue reversal for a partially settled batch")
    }
}
