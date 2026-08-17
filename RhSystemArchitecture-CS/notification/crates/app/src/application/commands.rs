//! Command inputs accepted by the notification use cases.

use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct SendNotificationCommand {
    pub recipient_id: Uuid,
    pub channel: String,
    pub subject: Option<String>,
    pub body: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RenderTemplateCommand {
    pub template_name: String,
    pub params: Vec<(String, String)>,
}
