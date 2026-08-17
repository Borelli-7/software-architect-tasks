//! Shared domain primitives for the Bank HRMS platform.
//!
//! Framework-free building blocks (identifiers, value objects, errors, ports)
//! reused by every microservice's domain layer.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

/// Errors raised by domain invariants and value-object validation.
#[derive(Debug, Error)]
pub enum DomainError {
    #[error("validation error: {0}")]
    Validation(String),
    #[error("entity not found: {0}")]
    NotFound(String),
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("invariant violated: {0}")]
    Invariant(String),
}

/// Monetary amount with an ISO-4217 currency code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    pub amount: Decimal,
    pub currency: String,
}

impl Money {
    pub fn new(amount: Decimal, currency: impl Into<String>) -> Self {
        Self { amount, currency: currency.into() }
    }
}

/// Validated e-mail value object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn parse(raw: impl Into<String>) -> Result<Self, DomainError> {
        let _raw = raw.into();
        todo!("validate against RFC 5322 and construct Email")
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Abstract wall clock; injected so use cases stay deterministic under test.
pub trait Clock: Send + Sync {
    fn now(&self) -> DateTime<Utc>;
}

#[derive(Debug, Default, Clone)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

/// Abstract identifier factory; swappable for deterministic ids in tests.
pub trait IdGenerator: Send + Sync {
    fn new_id(&self) -> Uuid;
}

#[derive(Debug, Default, Clone)]
pub struct UuidGenerator;

impl IdGenerator for UuidGenerator {
    fn new_id(&self) -> Uuid {
        Uuid::new_v4()
    }
}

/// Zero-based pagination request.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PageRequest {
    pub page: u32,
    pub size: u32,
}

impl Default for PageRequest {
    fn default() -> Self {
        Self { page: 0, size: 20 }
    }
}

/// Paginated query result wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagedResult<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u32,
    pub size: u32,
}
