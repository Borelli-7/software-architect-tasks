//! OIDC Authorization Code + PKCE flow configuration and steps.

use serde::{Deserialize, Serialize};

use crate::AuthError;
use crate::token::TokenSet;

/// Static OIDC configuration pointing at the bank's Keycloak realm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OidcConfig {
    pub issuer_url: String,
    pub client_id: String,
    pub redirect_uri: String,
    pub scopes: Vec<String>,
}

/// Drives the PKCE flow: build authorize URL, then exchange the code for tokens.
pub struct OidcClient {
    config: OidcConfig,
}

impl OidcClient {
    pub fn new(config: OidcConfig) -> Self {
        Self { config }
    }

    /// Builds the authorization request URL and the PKCE verifier to retain.
    pub fn begin_login(&self) -> Result<(String, String), AuthError> {
        todo!("generate PKCE verifier/challenge and construct the authorize URL")
    }

    /// Exchanges an authorization code + PKCE verifier for a token set.
    pub async fn complete_login(&self, _code: &str, _verifier: &str) -> Result<TokenSet, AuthError> {
        todo!("POST the token endpoint and parse the token response")
    }

    /// Uses a refresh token to obtain a fresh token set.
    pub async fn refresh(&self, _tokens: &TokenSet) -> Result<TokenSet, AuthError> {
        todo!("POST the token endpoint with grant_type=refresh_token")
    }
}
