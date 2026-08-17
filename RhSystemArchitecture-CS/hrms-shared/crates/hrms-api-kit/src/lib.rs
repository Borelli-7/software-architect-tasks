//! Shared Axum building blocks: the authenticated-user extractor and a common
//! pagination query. Reused by every service's presentation layer.

use axum::{extract::FromRequestParts, http::StatusCode, http::request::Parts};
use hrms_auth::SecurityContext;
use serde::Deserialize;

/// Extracts and verifies the caller's [`SecurityContext`] from the request.
pub struct AuthUser(pub SecurityContext);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(_parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        todo!("read bearer token, verify via JwtVerifier, build SecurityContext")
    }
}

/// Standard pagination query parameters (`?page=&size=`).
#[derive(Debug, Clone, Deserialize)]
pub struct Pagination {
    #[serde(default)]
    pub page: u32,
    #[serde(default = "default_size")]
    pub size: u32,
}

fn default_size() -> u32 {
    20
}
