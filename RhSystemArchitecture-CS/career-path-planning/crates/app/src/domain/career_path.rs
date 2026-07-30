//! The CareerPath aggregate: an ordered progression of roles with requirements.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::competency::RequiredCompetency;
use super::errors::CareerError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CareerPathId(pub Uuid);

/// A single role step within a career path.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CareerStep {
    pub role_title: String,
    pub level: u8,
    pub required_competencies: Vec<RequiredCompetency>,
}

/// Aggregate root: a defined career progression path.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CareerPath {
    pub id: CareerPathId,
    pub name: String,
    pub steps: Vec<CareerStep>,
}

impl CareerPath {
    /// Adds a step, enforcing monotonically increasing levels.
    pub fn add_step(&mut self, _step: CareerStep) -> Result<(), CareerError> {
        todo!("validate level ordering and append the step")
    }
}
