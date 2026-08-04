//! Global app state shared via Dioxus context.

use dioxus::prelude::*;

use hrms_api_client::ApiClient;

/// Authenticated user summary.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AuthState {
    pub user_id: Option<String>,
    pub email: Option<String>,
    pub roles: Vec<String>,
}

impl AuthState {
    pub fn is_authenticated(&self) -> bool {
        self.user_id.is_some()
    }
}

/// Cloneable handle to shared app state.
#[derive(Clone)]
pub struct AppState {
    pub api: ApiClient,
    pub auth: Signal<AuthState>,
}

/// Provides [`AppState`] into the Dioxus context tree. Call once at the root.
pub fn provide_app_state() {
    let state = AppState {
        api: ApiClient::new(crate::config::api_base_url()),
        auth: Signal::new(AuthState::default()),
    };
    use_context_provider(|| state);
}

/// Retrieves the shared [`AppState`].
pub fn use_app_state() -> AppState {
    use_context::<AppState>()
}
