//! HTTP adapter to an external training provider (LMS/catalog).

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::errors::TrainingError;
use crate::domain::ports::ExternalProviderClient;
use crate::domain::program::TrainingProgram;

pub struct HttpProviderClient {
    base_url: String,
}

impl HttpProviderClient {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }
}

#[async_trait]
impl ExternalProviderClient for HttpProviderClient {
    async fn register_enrollment(
        &self,
        _program: &TrainingProgram,
        _employee_id: Uuid,
    ) -> Result<(), TrainingError> {
        todo!("POST enrollment to the external provider API")
    }
}
