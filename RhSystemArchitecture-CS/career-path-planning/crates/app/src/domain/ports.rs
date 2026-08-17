//! Domain ports: persistence and the gap-analysis domain service.

use async_trait::async_trait;
use uuid::Uuid;

use super::career_path::{CareerPath, CareerPathId};
use super::competency::{EmployeeCompetency, GapAnalysis};
use super::errors::CareerError;

/// Persistence port for the CareerPath aggregate.
#[async_trait]
pub trait CareerPathRepository: Send + Sync {
    async fn find_by_id(&self, id: CareerPathId) -> Result<Option<CareerPath>, CareerError>;
    async fn save(&self, path: &CareerPath) -> Result<(), CareerError>;
}

/// Read port for an employee's assessed competencies.
#[async_trait]
pub trait CompetencyRepository: Send + Sync {
    async fn list_for_employee(&self, employee_id: Uuid) -> Result<Vec<EmployeeCompetency>, CareerError>;
}

/// Domain-service port computing the skill gap for a target career path.
#[async_trait]
pub trait GapAnalyzer: Send + Sync {
    async fn analyze(
        &self,
        employee_id: Uuid,
        path: &CareerPath,
        held: &[EmployeeCompetency],
    ) -> Result<GapAnalysis, CareerError>;
}
