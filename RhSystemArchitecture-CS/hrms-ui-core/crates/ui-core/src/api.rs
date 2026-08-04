//! Data-access helpers bridging the shared `ApiClient` to Dioxus hooks.

use crate::state::use_app_state;

/// Convenience accessor for the shared API client from within components.
pub fn api() -> hrms_api_client::ApiClient {
    use_app_state().api
}

// Per-service `use_resource` wrappers are added alongside each feature screen
// as its data loading is implemented.
