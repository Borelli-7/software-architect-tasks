//! Domain ports: persistence for the User aggregate.

use async_trait::async_trait;

use super::errors::IamError;
use super::user::{User, UserId};

/// Persistence port for the User aggregate.
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: UserId) -> Result<Option<User>, IamError>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, IamError>;
    async fn save(&self, user: &User) -> Result<(), IamError>;
}
