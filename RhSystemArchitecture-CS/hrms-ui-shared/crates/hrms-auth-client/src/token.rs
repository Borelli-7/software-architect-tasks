//! Token value types and expiry helpers.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessToken(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshToken(pub String);

/// The full set of tokens returned by the identity provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenSet {
    pub access_token: AccessToken,
    pub refresh_token: Option<RefreshToken>,
    pub expires_at: DateTime<Utc>,
    pub scopes: Vec<String>,
}

impl TokenSet {
    /// Whether the access token is past (or within a small skew of) expiry.
    pub fn is_expired(&self, _now: DateTime<Utc>) -> bool {
        todo!("compare now against expires_at including a refresh skew window")
    }
}
