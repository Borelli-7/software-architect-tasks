//! PostgreSQL adapter for message templates.

use async_trait::async_trait;
use hrms_persistence::DbPool;

use crate::domain::errors::NotificationError;
use crate::domain::ports::TemplateRepository;
use crate::domain::template::{Template, TemplateId};

pub struct PgTemplateRepository {
    pool: DbPool,
}

impl PgTemplateRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TemplateRepository for PgTemplateRepository {
    async fn find_by_id(&self, _id: TemplateId) -> Result<Option<Template>, NotificationError> {
        todo!("SELECT template by id")
    }

    async fn find_by_name(&self, _name: &str) -> Result<Option<Template>, NotificationError> {
        todo!("SELECT template by name")
    }
}
