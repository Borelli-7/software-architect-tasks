//! Score aggregation / trend analysis adapter.

use async_trait::async_trait;

use crate::domain::errors::EvaluationError;
use crate::domain::ports::AnalyticsEngine;
use crate::domain::review::Review;

#[derive(Default)]
pub struct WeightedAnalyticsEngine;

impl WeightedAnalyticsEngine {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl AnalyticsEngine for WeightedAnalyticsEngine {
    async fn overall_score(&self, _review: &Review) -> Result<f32, EvaluationError> {
        todo!("apply competency weights and compute the weighted overall score")
    }
}
