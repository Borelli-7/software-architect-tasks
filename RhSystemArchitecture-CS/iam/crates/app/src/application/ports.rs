//! Outbound application ports (implemented by infrastructure adapters).

use async_trait::async_trait;

use crate::domain::user::User;

/// Publishes identity lifecycle events to the message bus.
#[async_trait]
pub trait IamEventPublisher: Send + Sync {
    async fn user_created(&self, user: &User) -> anyhow::Result<()>;
    async fn role_assigned(&self, user: &User, role: &str) -> anyhow::Result<()>;
    async fn user_deactivated(&self, user: &User) -> anyhow::Result<()>;
}

/// Federates identities to the corporate directory / IdP (Keycloak, AD).
#[async_trait]
pub trait DirectoryFederation: Send + Sync {
    async fn provision(&self, user: &User) -> anyhow::Result<()>;
    async fn deprovision(&self, user: &User) -> anyhow::Result<()>;
}
