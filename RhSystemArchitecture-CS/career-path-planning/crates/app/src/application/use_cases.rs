//! Use case implementations, wiring the domain to injected outbound ports.

use std::sync::Arc;

use async_trait::async_trait;
use hrms_kernel::IdGenerator;

use super::CommandHandler;
use super::commands::{DefineCareerPathCommand, RunGapAnalysisCommand};
use super::dto::{CareerPathDto, GapAnalysisDto};
use super::errors::ApplicationError;
use super::ports::CareerEventPublisher;
use crate::domain::ports::{CareerPathRepository, CompetencyRepository, GapAnalyzer};

#[derive(Clone)]
pub struct DefineCareerPath {
    pub paths: Arc<dyn CareerPathRepository>,
    pub events: Arc<dyn CareerEventPublisher>,
    pub ids: Arc<dyn IdGenerator>,
}

#[async_trait]
impl CommandHandler<DefineCareerPathCommand> for DefineCareerPath {
    type Output = CareerPathDto;

    async fn handle(&self, _command: DefineCareerPathCommand) -> Result<Self::Output, ApplicationError> {
        todo!("create and persist a career path, publish CareerPathDefined")
    }
}

#[derive(Clone)]
pub struct RunGapAnalysis {
    pub paths: Arc<dyn CareerPathRepository>,
    pub competencies: Arc<dyn CompetencyRepository>,
    pub analyzer: Arc<dyn GapAnalyzer>,
    pub events: Arc<dyn CareerEventPublisher>,
}

#[async_trait]
impl CommandHandler<RunGapAnalysisCommand> for RunGapAnalysis {
    type Output = GapAnalysisDto;

    async fn handle(&self, _command: RunGapAnalysisCommand) -> Result<Self::Output, ApplicationError> {
        todo!("load path + employee competencies, run analyzer, publish GapAnalysisCompleted")
    }
}
