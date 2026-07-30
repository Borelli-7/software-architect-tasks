//! PostgreSQL adapter for the Notification aggregate.

use async_trait::async_trait;
use hrms_persistence::DbPool;

use crate::domain::errors::NotificationError;
use crate::domain::notification::{Notification, NotificationId};
use crate::domain::ports::NotificationRepository;

pub struct PgNotificationRepository {
    pool: DbPool,
}

impl PgNotificationRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl NotificationRepository for PgNotificationRepository {
    async fn find_by_id(&self, _id: NotificationId) -> Result<Option<Notification>, NotificationError> {
        todo!("SELECT notification by id")
    }

    async fn save(&self, _notification: &Notification) -> Result<(), NotificationError> {
        todo!("UPSERT notification row")
    }
}
