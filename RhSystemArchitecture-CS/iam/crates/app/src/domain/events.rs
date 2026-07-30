//! Domain events emitted by the user aggregate.

use serde::{Deserialize, Serialize};

use super::user::UserId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IamEvent {
    UserCreated { id: UserId },
    RoleAssigned { id: UserId, role: String },
    UserDeactivated { id: UserId },
}
