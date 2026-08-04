//! Notification DTOs.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationDto {
    pub id: Uuid,
    pub channel: String,
    pub recipient: String,
    pub subject: Option<String>,
    pub body: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendNotificationRequest {
    pub channel: String,
    pub recipient: String,
    pub subject: Option<String>,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderTemplateRequest {
    pub template_key: String,
    pub variables: serde_json::Value,
}
