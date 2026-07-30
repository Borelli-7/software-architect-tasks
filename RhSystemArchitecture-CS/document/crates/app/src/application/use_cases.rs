//! Use case implementations, wiring the domain to injected outbound ports.

use std::sync::Arc;

use async_trait::async_trait;
use hrms_kernel::{Clock, IdGenerator};

use super::CommandHandler;
use super::commands::{GenerateDocumentCommand, RetrieveDocumentCommand, UploadDocumentCommand};
use super::dto::{DocumentContentDto, DocumentDto};
use super::errors::ApplicationError;
use super::ports::{BlobStore, DocumentEventPublisher, DocumentGenerator};
use crate::domain::ports::DocumentRepository;

#[derive(Clone)]
pub struct UploadDocument {
    pub documents: Arc<dyn DocumentRepository>,
    pub blobs: Arc<dyn BlobStore>,
    pub events: Arc<dyn DocumentEventPublisher>,
    pub clock: Arc<dyn Clock>,
    pub ids: Arc<dyn IdGenerator>,
}

#[async_trait]
impl CommandHandler<UploadDocumentCommand> for UploadDocument {
    type Output = DocumentDto;

    async fn handle(&self, _command: UploadDocumentCommand) -> Result<Self::Output, ApplicationError> {
        todo!("validate, store bytes in blob store, persist metadata, publish Uploaded")
    }
}

#[derive(Clone)]
pub struct GenerateDocument {
    pub documents: Arc<dyn DocumentRepository>,
    pub blobs: Arc<dyn BlobStore>,
    pub generator: Arc<dyn DocumentGenerator>,
    pub events: Arc<dyn DocumentEventPublisher>,
    pub clock: Arc<dyn Clock>,
    pub ids: Arc<dyn IdGenerator>,
}

#[async_trait]
impl CommandHandler<GenerateDocumentCommand> for GenerateDocument {
    type Output = DocumentDto;

    async fn handle(&self, _command: GenerateDocumentCommand) -> Result<Self::Output, ApplicationError> {
        todo!("render template, store bytes, persist metadata, publish Generated")
    }
}

#[derive(Clone)]
pub struct RetrieveDocument {
    pub documents: Arc<dyn DocumentRepository>,
    pub blobs: Arc<dyn BlobStore>,
}

#[async_trait]
impl CommandHandler<RetrieveDocumentCommand> for RetrieveDocument {
    type Output = DocumentContentDto;

    async fn handle(&self, _command: RetrieveDocumentCommand) -> Result<Self::Output, ApplicationError> {
        todo!("load metadata, fetch bytes from blob store, return content")
    }
}
