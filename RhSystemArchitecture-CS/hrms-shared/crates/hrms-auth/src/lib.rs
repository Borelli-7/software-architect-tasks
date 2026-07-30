//! Authentication & authorization primitives shared across services.
//!
//! Provides the security context carried on every request plus the RBAC/ABAC
//! trait surface. Concrete policy evaluation is left to implementers.

use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("invalid or expired token")]
    InvalidToken,
    #[error("access denied")]
    Denied,
}

/// Coarse organizational roles (RBAC). Fine-grained rights use [`Permission`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Role {
    Employee,
    Supervisor,
    HrAdmin,
    HrManager,
    HrPayroll,
    SystemAdmin,
}

/// Fine-grained permission string, e.g. `payroll:run`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Permission(pub String);

/// Action attempted against a protected resource.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {
    Read,
    Create,
    Update,
    Delete,
    Approve,
}

/// Authenticated caller identity + attributes, propagated per request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityContext {
    pub user_id: Uuid,
    pub roles: HashSet<Role>,
    pub permissions: HashSet<Permission>,
    pub branch_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
}

impl SecurityContext {
    pub fn has_role(&self, role: &Role) -> bool {
        self.roles.contains(role)
    }
}

/// Attribute descriptor for the resource being accessed (ABAC input).
#[derive(Debug, Clone)]
pub struct ResourceRef {
    pub kind: String,
    pub owner_id: Option<Uuid>,
    pub branch_id: Option<Uuid>,
}

/// Role-based decision surface.
pub trait AuthorizationPolicy: Send + Sync {
    fn is_allowed(&self, ctx: &SecurityContext, action: &Action, resource: &ResourceRef) -> bool;
}

/// Attribute-based rule evaluated after RBAC narrows access.
pub trait AbacRule: Send + Sync {
    fn evaluate(&self, ctx: &SecurityContext, action: &Action, resource: &ResourceRef) -> bool;
}

/// Default RBAC policy backed by a role/action matrix.
#[derive(Default)]
pub struct RbacPolicy;

impl AuthorizationPolicy for RbacPolicy {
    fn is_allowed(&self, _ctx: &SecurityContext, _action: &Action, _resource: &ResourceRef) -> bool {
        todo!("evaluate role/action/resource matrix")
    }
}

/// Composed authorization entry point (RBAC + ABAC).
pub trait Authorizer: Send + Sync {
    fn authorize(
        &self,
        ctx: &SecurityContext,
        action: &Action,
        resource: &ResourceRef,
    ) -> Result<(), AuthError>;
}

/// JWT claim set issued by the identity provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub roles: Vec<Role>,
    pub exp: usize,
}

/// Verifies a bearer token and materializes a [`SecurityContext`].
pub trait JwtVerifier: Send + Sync {
    fn verify(&self, token: &str) -> Result<SecurityContext, AuthError>;
}
