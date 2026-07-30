//! Use case implementations, wiring the domain to injected outbound ports.

use std::sync::Arc;

use async_trait::async_trait;
use hrms_kernel::{Clock, IdGenerator};

use super::CommandHandler;
use super::commands::{QueryAuditLogCommand, RecordAuditEntryCommand};
use super::dto::AuditEntryDto;
use super::errors::ApplicationError;
use super::ports::AuditEventPublisher;
use crate::domain::ports::AuditRepository;

#[derive(Clone)]
pub struct RecordAuditEntry {
    pub entries: Arc<dyn AuditRepository>,
    pub events: Arc<dyn AuditEventPublisher>,
    pub clock: Arc<dyn Clock>,
    pub ids: Arc<dyn IdGenerator>,
}

#[async_trait]
impl CommandHandler<RecordAuditEntryCommand> for RecordAuditEntry {
    type Output = AuditEntryDto;

    async fn handle(&self, _command: RecordAuditEntryCommand) -> Result<Self::Output, ApplicationError> {
        todo!("build entry, validate, append to log, publish Recorded")
    }
}

#[derive(Clone)]
pub struct QueryAuditLog {
    pub entries: Arc<dyn AuditRepository>,
}

#[async_trait]
impl CommandHandler<QueryAuditLogCommand> for QueryAuditLog {
    type Output = Vec<AuditEntryDto>;

    async fn handle(&self, _command: QueryAuditLogCommand) -> Result<Self::Output, ApplicationError> {
        todo!("map command -> AuditQuery, run query, map results")
    }
}
