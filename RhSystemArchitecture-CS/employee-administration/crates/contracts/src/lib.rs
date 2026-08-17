//! Public event contract for the Employee Administration service.
//!
//! Other services depend on this crate to consume `employee.lifecycle` events
//! without coupling to the service's internal domain model.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Kafka topic carrying employee lifecycle events.
pub const TOPIC: &str = "employee.lifecycle";

/// Tagged union of employee lifecycle events published by this service.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EmployeeLifecycleEvent {
    EmployeeCreated(EmployeeCreated),
    EmployeeUpdated(EmployeeUpdated),
    EmployeeTerminated(EmployeeTerminated),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeCreated {
    pub id: Uuid,
    pub branch_id: Uuid,
    pub department_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeUpdated {
    pub id: Uuid,
    pub department_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeTerminated {
    pub id: Uuid,
}
