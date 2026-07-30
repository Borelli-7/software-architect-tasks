//! Use case implementations, wiring the domain to injected outbound ports.

use std::sync::Arc;

use async_trait::async_trait;
use hrms_kernel::{Clock, IdGenerator};

use super::CommandHandler;
use super::commands::{AssignRoleCommand, CreateUserCommand, DeactivateUserCommand};
use super::dto::UserDto;
use super::errors::ApplicationError;
use super::ports::{DirectoryFederation, IamEventPublisher};
use crate::domain::ports::UserRepository;

#[derive(Clone)]
pub struct CreateUser {
    pub users: Arc<dyn UserRepository>,
    pub directory: Arc<dyn DirectoryFederation>,
    pub events: Arc<dyn IamEventPublisher>,
    pub clock: Arc<dyn Clock>,
    pub ids: Arc<dyn IdGenerator>,
}

#[async_trait]
impl CommandHandler<CreateUserCommand> for CreateUser {
    type Output = UserDto;

    async fn handle(&self, _command: CreateUserCommand) -> Result<Self::Output, ApplicationError> {
        todo!("validate uniqueness, create user, provision directory identity, publish UserCreated")
    }
}

#[derive(Clone)]
pub struct AssignRole {
    pub users: Arc<dyn UserRepository>,
    pub events: Arc<dyn IamEventPublisher>,
}

#[async_trait]
impl CommandHandler<AssignRoleCommand> for AssignRole {
    type Output = UserDto;

    async fn handle(&self, _command: AssignRoleCommand) -> Result<Self::Output, ApplicationError> {
        todo!("load user, assign role, persist, publish RoleAssigned")
    }
}

#[derive(Clone)]
pub struct DeactivateUser {
    pub users: Arc<dyn UserRepository>,
    pub directory: Arc<dyn DirectoryFederation>,
    pub events: Arc<dyn IamEventPublisher>,
}

#[async_trait]
impl CommandHandler<DeactivateUserCommand> for DeactivateUser {
    type Output = UserDto;

    async fn handle(&self, _command: DeactivateUserCommand) -> Result<Self::Output, ApplicationError> {
        todo!("deactivate user, deprovision directory identity, publish UserDeactivated")
    }
}
