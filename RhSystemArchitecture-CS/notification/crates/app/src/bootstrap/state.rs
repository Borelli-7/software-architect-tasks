//! Axum shared state: the wired use cases exposed to handlers.

use std::sync::Arc;

use crate::application::use_cases::{RenderTemplate, SendNotification};

#[derive(Clone)]
pub struct AppState {
    pub send_notification: Arc<SendNotification>,
    pub render_template: Arc<RenderTemplate>,
}
