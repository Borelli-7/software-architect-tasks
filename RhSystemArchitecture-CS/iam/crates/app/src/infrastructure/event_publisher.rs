//! Kafka adapter publishing iam lifecycle events to the message bus.

use std::sync::Arc;

use async_trait::async_trait;
use hrms_messaging::EventPublisher;

use crate::application::ports::IamEventPublisher;
use crate::domain::user::User;

pub struct KafkaIamEventPublisher {
    publisher: Arc<dyn EventPublisher>,
    topic: String,
}

impl KafkaIamEventPublisher {
    pub fn new(publisher: Arc<dyn EventPublisher>, topic: String) -> Self {
        Self { publisher, topic }
    }
}

#[async_trait]
impl IamEventPublisher for KafkaIamEventPublisher {
    async fn user_created(&self, _user: &User) -> anyhow::Result<()> {
        todo!("map to UserCreated contract and publish to topic")
    }

    async fn role_assigned(&self, _user: &User, _role: &str) -> anyhow::Result<()> {
        todo!("map to RoleAssigned contract and publish to topic")
    }

    async fn user_deactivated(&self, _user: &User) -> anyhow::Result<()> {
        todo!("map to UserDeactivated contract and publish to topic")
    }
}
