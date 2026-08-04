//! Platform-agnostic token persistence contract.

use crate::AuthError;
use crate::token::TokenSet;

/// Persists the active [`TokenSet`]. Implemented per platform (browser storage,
/// OS keychain, mobile secure enclave).
pub trait TokenStore: Send + Sync {
    fn load(&self) -> Result<Option<TokenSet>, AuthError>;
    fn save(&self, tokens: &TokenSet) -> Result<(), AuthError>;
    fn clear(&self) -> Result<(), AuthError>;
}
