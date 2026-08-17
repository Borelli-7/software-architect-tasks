//! Public event contract for the Payroll service.
//!
//! Consumed by other services (reporting, notification) that react to payroll
//! outcomes without depending on the payroll domain model.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Kafka topic carrying payroll lifecycle events.
pub const TOPIC: &str = "payroll.lifecycle";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PayrollLifecycleEvent {
    PayrollApproved(PayrollApproved),
    PayrollPaid(PayrollPaid),
    PayrollFailed(PayrollFailed),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayrollApproved {
    pub run_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayrollPaid {
    pub run_id: Uuid,
    pub reference: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayrollFailed {
    pub run_id: Uuid,
    pub reason: String,
}
