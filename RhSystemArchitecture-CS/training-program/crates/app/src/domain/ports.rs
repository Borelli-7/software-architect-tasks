//! Domain ports: persistence and the external provider integration.

use async_trait::async_trait;

use super::errors::TrainingError;
use super::program::{ProgramId, TrainingProgram};

/// Persistence port for the TrainingProgram aggregate.
#[async_trait]
pub trait TrainingRepository: Send + Sync {
    async fn find_by_id(&self, id: ProgramId) -> Result<Option<TrainingProgram>, TrainingError>;
    async fn save(&self, program: &TrainingProgram) -> Result<(), TrainingError>;
}

/// Port to an external training provider (LMS/catalog) for enrollment sync.
#[async_trait]
pub trait ExternalProviderClient: Send + Sync {
    async fn register_enrollment(
        &self,
        program: &TrainingProgram,
        employee_id: uuid::Uuid,
    ) -> Result<(), TrainingError>;
}
