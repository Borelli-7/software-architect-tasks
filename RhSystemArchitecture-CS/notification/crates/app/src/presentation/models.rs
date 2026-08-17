//! HTTP request/response models for the notification resources.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct SendNotificationRequest {
    pub recipient_id: Uuid,
    pub channel: String,
    pub subject: Option<String>,
    pub body: String,
}

#[derive(Debug, Deserialize)]
pub struct RenderTemplateRequest {
    pub template_name: String,
    pub params: Vec<(String, String)>,
}

#[derive(Debug, Serialize)]
pub struct NotificationResponse {
    pub id: Uuid,
    pub recipient_id: Uuid,
    pub channel: String,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct RenderedTemplateResponse {
    pub subject: Option<String>,
    pub body: String,
}
