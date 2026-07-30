//! Use case implementations, wiring the domain to injected outbound ports.

use std::sync::Arc;

use async_trait::async_trait;
use hrms_kernel::{Clock, IdGenerator};

use super::CommandHandler;
use super::commands::{
    CloseCampaignCommand, OpenCampaignCommand, SetObjectivesCommand, SubmitReviewCommand,
};
use super::dto::{CampaignDto, ReviewDto};
use super::errors::ApplicationError;
use super::ports::EvaluationEventPublisher;
use crate::domain::ports::{
    AnalyticsEngine, CampaignRepository, ObjectiveRepository, ReviewRepository,
};

#[derive(Clone)]
pub struct OpenCampaign {
    pub campaigns: Arc<dyn CampaignRepository>,
    pub events: Arc<dyn EvaluationEventPublisher>,
    pub clock: Arc<dyn Clock>,
    pub ids: Arc<dyn IdGenerator>,
}

#[async_trait]
impl CommandHandler<OpenCampaignCommand> for OpenCampaign {
    type Output = CampaignDto;

    async fn handle(&self, _command: OpenCampaignCommand) -> Result<Self::Output, ApplicationError> {
        todo!("create campaign, transition to Open, persist, publish CampaignOpened")
    }
}

#[derive(Clone)]
pub struct SubmitReview {
    pub reviews: Arc<dyn ReviewRepository>,
    pub analytics: Arc<dyn AnalyticsEngine>,
    pub events: Arc<dyn EvaluationEventPublisher>,
}

#[async_trait]
impl CommandHandler<SubmitReviewCommand> for SubmitReview {
    type Output = ReviewDto;

    async fn handle(&self, _command: SubmitReviewCommand) -> Result<Self::Output, ApplicationError> {
        todo!("submit review, compute overall score via analytics, publish EvaluationCompleted")
    }
}

#[derive(Clone)]
pub struct SetObjectives {
    pub objectives: Arc<dyn ObjectiveRepository>,
    pub ids: Arc<dyn IdGenerator>,
}

#[async_trait]
impl CommandHandler<SetObjectivesCommand> for SetObjectives {
    type Output = ();

    async fn handle(&self, _command: SetObjectivesCommand) -> Result<Self::Output, ApplicationError> {
        todo!("persist objectives for the employee/period")
    }
}

#[derive(Clone)]
pub struct CloseCampaign {
    pub campaigns: Arc<dyn CampaignRepository>,
    pub events: Arc<dyn EvaluationEventPublisher>,
}

#[async_trait]
impl CommandHandler<CloseCampaignCommand> for CloseCampaign {
    type Output = CampaignDto;

    async fn handle(&self, _command: CloseCampaignCommand) -> Result<Self::Output, ApplicationError> {
        todo!("validate all reviews complete, transition to Closed, persist")
    }
}
