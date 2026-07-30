//! Use case implementations. Dependencies are injected as trait objects so the
//! application layer stays free of concrete infrastructure.

use std::sync::Arc;

use async_trait::async_trait;
use hrms_kernel::{Clock, IdGenerator};

use super::CommandHandler;
use super::commands::{CreateEmployeeCommand, TerminateEmployeeCommand, UpdateEmployeeCommand};
use super::dto::EmployeeDto;
use super::errors::ApplicationError;
use super::ports::{EmployeeEventPublisher, IdentityProvisioner};
use crate::domain::ports::EmployeeRepository;

#[derive(Clone)]
pub struct CreateEmployee {
    pub repository: Arc<dyn EmployeeRepository>,
    pub events: Arc<dyn EmployeeEventPublisher>,
    pub identity: Arc<dyn IdentityProvisioner>,
    pub clock: Arc<dyn Clock>,
    pub ids: Arc<dyn IdGenerator>,
}

#[async_trait]
impl CommandHandler<CreateEmployeeCommand> for CreateEmployee {
    type Output = EmployeeDto;

    async fn handle(&self, _command: CreateEmployeeCommand) -> Result<Self::Output, ApplicationError> {
        todo!("validate input, build Employee, persist, provision identity, publish EmployeeCreated")
    }
}

#[derive(Clone)]
pub struct UpdateEmployee {
    pub repository: Arc<dyn EmployeeRepository>,
}

#[async_trait]
impl CommandHandler<UpdateEmployeeCommand> for UpdateEmployee {
    type Output = EmployeeDto;

    async fn handle(&self, _command: UpdateEmployeeCommand) -> Result<Self::Output, ApplicationError> {
        todo!("load aggregate, apply changes, persist")
    }
}

#[derive(Clone)]
pub struct TerminateEmployee {
    pub repository: Arc<dyn EmployeeRepository>,
    pub events: Arc<dyn EmployeeEventPublisher>,
    pub identity: Arc<dyn IdentityProvisioner>,
    pub clock: Arc<dyn Clock>,
}

#[async_trait]
impl CommandHandler<TerminateEmployeeCommand> for TerminateEmployee {
    type Output = ();

    async fn handle(&self, _command: TerminateEmployeeCommand) -> Result<Self::Output, ApplicationError> {
        todo!("load aggregate, terminate, persist, deprovision identity, publish EmployeeTerminated")
    }
}
