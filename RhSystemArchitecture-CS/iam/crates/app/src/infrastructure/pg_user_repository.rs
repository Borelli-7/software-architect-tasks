//! PostgreSQL adapter for the User aggregate.

use async_trait::async_trait;
use hrms_persistence::DbPool;

use crate::domain::errors::IamError;
use crate::domain::ports::UserRepository;
use crate::domain::user::{User, UserId};

pub struct PgUserRepository {
    pool: DbPool,
}

impl PgUserRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn find_by_id(&self, _id: UserId) -> Result<Option<User>, IamError> {
        todo!("SELECT user + roles by id")
    }

    async fn find_by_username(&self, _username: &str) -> Result<Option<User>, IamError> {
        todo!("SELECT user by username")
    }

    async fn save(&self, _user: &User) -> Result<(), IamError> {
        todo!("UPSERT user + role grants")
    }
}
