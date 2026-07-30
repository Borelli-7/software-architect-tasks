//! Axum shared state: the wired use cases exposed to handlers.

use std::sync::Arc;

use crate::application::use_cases::{GenerateDocument, RetrieveDocument, UploadDocument};

#[derive(Clone)]
pub struct AppState {
    pub upload_document: Arc<UploadDocument>,
    pub generate_document: Arc<GenerateDocument>,
    pub retrieve_document: Arc<RetrieveDocument>,
}
