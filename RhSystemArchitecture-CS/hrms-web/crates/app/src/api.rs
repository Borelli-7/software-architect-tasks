//! Reactive data-access helpers bridging the shared `ApiClient` to Leptos
//! resources. Feature views call these instead of touching `ApiClient` directly.

use crate::state::use_app_state;

/// Convenience accessor for the shared API client from within components.
pub fn api() -> hrms_api_client::ApiClient {
    use_app_state().api
}

// Per-service resource wrappers (LocalResource so futures run on the wasm client)
// are added alongside each feature module as its screens are implemented.
