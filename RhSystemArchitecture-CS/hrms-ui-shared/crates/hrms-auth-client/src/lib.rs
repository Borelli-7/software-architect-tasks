//! Framework-agnostic OIDC/token client shared by all frontends.
//!
//! Encapsulates the OIDC Authorization Code + PKCE flow and token lifecycle.
//! Platform-specific storage (browser `localStorage`, OS keychain, mobile secure
//! store) is provided by consumers through the [`TokenStore`] trait.
#![allow(dead_code)]

pub mod oidc;
pub mod store;
pub mod token;

pub use oidc::{OidcClient, OidcConfig};
pub use store::TokenStore;
pub use token::{AccessToken, RefreshToken, TokenSet};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("no active session")]
    NoSession,
    #[error("token expired")]
    Expired,
    #[error("oidc flow failed: {0}")]
    Flow(String),
    #[error("token store failure: {0}")]
    Store(String),
}
