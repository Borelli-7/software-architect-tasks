//! Typed REST client for the Bank HRMS API gateway, shared by every frontend.
//!
//! [`ApiClient`] holds the gateway base URL and an optional bearer token; the
//! `dto` modules mirror the backend service contracts, and the `endpoints`
//! modules expose typed async methods (bodies stubbed with `todo!()`).
#![allow(dead_code)]

pub mod dto;
pub mod endpoints;
pub mod error;

pub use error::ApiError;

use std::sync::{Arc, RwLock};

/// Thin, cloneable REST client. Cloning shares the underlying reqwest client
/// and token cell, so it can be stored in UI context and passed freely.
#[derive(Clone)]
pub struct ApiClient {
    base_url: Arc<str>,
    http: reqwest::Client,
    bearer: Arc<RwLock<Option<String>>>,
}

impl ApiClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: Arc::from(base_url.into()),
            http: reqwest::Client::new(),
            bearer: Arc::new(RwLock::new(None)),
        }
    }

    /// Sets (or clears) the bearer token used for subsequent requests.
    pub fn set_bearer(&self, token: Option<String>) {
        *self.bearer.write().expect("bearer lock poisoned") = token;
    }

    /// Builds an absolute URL for a gateway-relative path.
    pub(crate) fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    /// Current bearer token, if any.
    pub(crate) fn bearer(&self) -> Option<String> {
        self.bearer.read().expect("bearer lock poisoned").clone()
    }

    pub(crate) fn http(&self) -> &reqwest::Client {
        &self.http
    }
}
