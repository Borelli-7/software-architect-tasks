//! Use case implementations, wiring the domain to injected outbound ports.

use std::sync::Arc;

use async_trait::async_trait;
use hrms_kernel::{Clock, IdGenerator};

use super::CommandHandler;
use super::commands::{RenderTemplateCommand, SendNotificationCommand};
use super::dto::{NotificationDto, RenderedTemplateDto};
use super::errors::ApplicationError;
use super::ports::{ChannelGateway, NotificationEventPublisher};
use crate::domain::ports::{NotificationRepository, TemplateRepository};

#[derive(Clone)]
pub struct SendNotification {
    pub notifications: Arc<dyn NotificationRepository>,
    pub gateways: Vec<Arc<dyn ChannelGateway>>,
    pub events: Arc<dyn NotificationEventPublisher>,
    pub clock: Arc<dyn Clock>,
    pub ids: Arc<dyn IdGenerator>,
}

#[async_trait]
impl CommandHandler<SendNotificationCommand> for SendNotification {
    type Output = NotificationDto;

    async fn handle(&self, _command: SendNotificationCommand) -> Result<Self::Output, ApplicationError> {
        todo!("build notification, select gateway, deliver, mark status, publish event")
    }
}

#[derive(Clone)]
pub struct RenderTemplate {
    pub templates: Arc<dyn TemplateRepository>,
}

#[async_trait]
impl CommandHandler<RenderTemplateCommand> for RenderTemplate {
    type Output = RenderedTemplateDto;

    async fn handle(&self, _command: RenderTemplateCommand) -> Result<Self::Output, ApplicationError> {
        todo!("load template by name, render against params, return body")
    }
}
