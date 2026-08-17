//! Axum shared state: the wired use cases exposed to handlers.

use std::sync::Arc;

use crate::application::use_cases::{ApprovePayroll, RunPayroll, SubmitPaymentBatch};

#[derive(Clone)]
pub struct AppState {
    pub run_payroll: Arc<RunPayroll>,
    pub approve_payroll: Arc<ApprovePayroll>,
    pub submit_payment_batch: Arc<SubmitPaymentBatch>,
}
