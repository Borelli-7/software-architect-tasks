//! Persistence abstractions shared by service infrastructure layers.
//!
//! Defines the generic repository, unit-of-work, and transaction-manager ports
//! plus the concrete PostgreSQL pool type used across services.

use async_trait::async_trait;
use thiserror::Error;

/// Concrete connection pool type (sqlx + PostgreSQL).
pub type DbPool = sqlx::PgPool;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("entity not found")]
    NotFound,
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
}

/// Generic aggregate repository port.
#[async_trait]
pub trait Repository<T, Id>: Send + Sync {
    async fn find_by_id(&self, id: Id) -> Result<Option<T>, RepositoryError>;
    async fn save(&self, entity: &T) -> Result<(), RepositoryError>;
    async fn delete(&self, id: Id) -> Result<(), RepositoryError>;
}

/// Transactional boundary handle.
#[async_trait]
pub trait UnitOfWork: Send + Sync {
    async fn commit(self: Box<Self>) -> Result<(), RepositoryError>;
    async fn rollback(self: Box<Self>) -> Result<(), RepositoryError>;
}

/// Opens transactional units of work.
#[async_trait]
pub trait TransactionManager: Send + Sync {
    async fn begin(&self) -> Result<Box<dyn UnitOfWork>, RepositoryError>;
}

/// Builds a PostgreSQL connection pool.
pub async fn connect(url: &str, max_connections: u32) -> Result<DbPool, sqlx::Error> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(url)
        .await
}
