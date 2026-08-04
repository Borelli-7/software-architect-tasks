//! Global app state provided via Leptos context.

use leptos::prelude::*;

use hrms_api_client::ApiClient;

/// Authenticated user summary held in reactive state.
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

/// Cloneable handle to shared reactive app state.
#[derive(Clone)]
pub struct AppState {
    pub api: ApiClient,
    pub auth: RwSignal<AuthState>,
}

/// Provides [`AppState`] into the reactive context tree.
pub fn provide_app_state() {
    let state = AppState {
        api: ApiClient::new(crate::config::api_base_url()),
        auth: RwSignal::new(AuthState::default()),
    };
    provide_context(state);
}

/// Retrieves the shared [`AppState`]; panics if not provided by an ancestor.
pub fn use_app_state() -> AppState {
    use_context::<AppState>().expect("AppState must be provided at the app root")
}
