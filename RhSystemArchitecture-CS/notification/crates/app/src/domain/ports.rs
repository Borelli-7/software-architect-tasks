//! Domain ports: persistence for notifications and templates.

use async_trait::async_trait;

use super::errors::NotificationError;
use super::notification::{Notification, NotificationId};
use super::template::{Template, TemplateId};

#[async_trait]
pub trait NotificationRepository: Send + Sync {
    async fn find_by_id(&self, id: NotificationId) -> Result<Option<Notification>, NotificationError>;
    async fn save(&self, notification: &Notification) -> Result<(), NotificationError>;
}

#[async_trait]
pub trait TemplateRepository: Send + Sync {
    async fn find_by_id(&self, id: TemplateId) -> Result<Option<Template>, NotificationError>;
    async fn find_by_name(&self, name: &str) -> Result<Option<Template>, NotificationError>;
}
