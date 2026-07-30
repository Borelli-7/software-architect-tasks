//! The User aggregate: an identity with assigned roles.

use chrono::{DateTime, Utc};
use hrms_auth::Role;
use hrms_kernel::Email;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::errors::IamError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub Uuid);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserStatus {
    Active,
    Suspended,
    Deactivated,
}

/// Aggregate root: a platform user account and its role grants.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub email: Email,
    pub status: UserStatus,
    pub roles: Vec<Role>,
    pub created_at: DateTime<Utc>,
}

impl User {
    /// Grants a role, ignoring duplicates.
    pub fn assign_role(&mut self, _role: Role) -> Result<(), IamError> {
        todo!("add role if not already present")
    }

    /// Revokes a role, enforcing that at least the baseline access remains valid.
    pub fn revoke_role(&mut self, _role: &Role) -> Result<(), IamError> {
        todo!("remove role and validate remaining grants")
    }

    /// Deactivates the account (blocks future authentication).
    pub fn deactivate(&mut self) -> Result<(), IamError> {
        todo!("guard status and transition to Deactivated")
    }
}
