//! Default gap-analysis engine comparing held vs. required competency levels.

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::career_path::CareerPath;
use crate::domain::competency::{EmployeeCompetency, GapAnalysis};
use crate::domain::errors::CareerError;
use crate::domain::ports::GapAnalyzer;

#[derive(Default)]
pub struct LevelGapAnalyzer;

impl LevelGapAnalyzer {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl GapAnalyzer for LevelGapAnalyzer {
    async fn analyze(
        &self,
        _employee_id: Uuid,
        _path: &CareerPath,
        _held: &[EmployeeCompetency],
    ) -> Result<GapAnalysis, CareerError> {
        todo!("diff required vs. held competency levels into a GapAnalysis")
    }
}
