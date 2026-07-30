//! Use case implementations, wiring the domain to injected outbound ports.

use std::sync::Arc;

use async_trait::async_trait;
use hrms_kernel::{Clock, IdGenerator};

use super::CommandHandler;
use super::commands::{CompleteTrainingCommand, CreateProgramCommand, EnrollEmployeeCommand};
use super::dto::ProgramDto;
use super::errors::ApplicationError;
use super::ports::TrainingEventPublisher;
use crate::domain::ports::{ExternalProviderClient, TrainingRepository};

#[derive(Clone)]
pub struct CreateProgram {
    pub programs: Arc<dyn TrainingRepository>,
    pub ids: Arc<dyn IdGenerator>,
}

#[async_trait]
impl CommandHandler<CreateProgramCommand> for CreateProgram {
    type Output = ProgramDto;

    async fn handle(&self, _command: CreateProgramCommand) -> Result<Self::Output, ApplicationError> {
        todo!("create and persist a training program")
    }
}

#[derive(Clone)]
pub struct EnrollEmployee {
    pub programs: Arc<dyn TrainingRepository>,
    pub provider: Arc<dyn ExternalProviderClient>,
    pub events: Arc<dyn TrainingEventPublisher>,
    pub ids: Arc<dyn IdGenerator>,
}

#[async_trait]
impl CommandHandler<EnrollEmployeeCommand> for EnrollEmployee {
    type Output = ProgramDto;

    async fn handle(&self, _command: EnrollEmployeeCommand) -> Result<Self::Output, ApplicationError> {
        todo!("enroll employee, sync to external provider, publish EmployeeEnrolled")
    }
}

#[derive(Clone)]
pub struct CompleteTraining {
    pub programs: Arc<dyn TrainingRepository>,
    pub events: Arc<dyn TrainingEventPublisher>,
    pub clock: Arc<dyn Clock>,
}

#[async_trait]
impl CommandHandler<CompleteTrainingCommand> for CompleteTraining {
    type Output = ProgramDto;

    async fn handle(&self, _command: CompleteTrainingCommand) -> Result<Self::Output, ApplicationError> {
        todo!("mark enrollment complete, issue certificate, publish TrainingCompleted")
    }
}
