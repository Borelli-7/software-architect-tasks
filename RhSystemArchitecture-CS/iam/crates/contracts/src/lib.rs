//! Public event contract for the Identity & Access Management service.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Kafka topic carrying identity lifecycle events.
pub const TOPIC: &str = "iam.lifecycle";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum IamLifecycleEvent {
    UserCreated(UserCreated),
    RoleAssigned(RoleAssigned),
    UserDeactivated(UserDeactivated),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCreated {
    pub user_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleAssigned {
    pub user_id: Uuid,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDeactivated {
    pub user_id: Uuid,
}
