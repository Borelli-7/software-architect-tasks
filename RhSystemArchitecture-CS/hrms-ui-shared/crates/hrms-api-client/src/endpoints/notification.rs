//! Notification endpoints (`/notifications`, `/templates/render`).

use crate::ApiClient;
use crate::dto::notification::{NotificationDto, RenderTemplateRequest, SendNotificationRequest};
use crate::error::ApiError;

pub struct NotificationApi<'a>(pub(crate) &'a ApiClient);

impl NotificationApi<'_> {
    pub async fn send(&self, _req: SendNotificationRequest) -> Result<NotificationDto, ApiError> {
        todo!("POST /notifications")
    }

    pub async fn render_template(&self, _req: RenderTemplateRequest) -> Result<String, ApiError> {
        todo!("POST /templates/render")
    }
}
