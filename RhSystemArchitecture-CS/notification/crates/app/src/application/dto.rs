//! Data-transfer objects returned by notification use cases.

use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct NotificationDto {
    pub id: Uuid,
    pub recipient_id: Uuid,
    pub channel: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RenderedTemplateDto {
    pub subject: Option<String>,
    pub body: String,
}
