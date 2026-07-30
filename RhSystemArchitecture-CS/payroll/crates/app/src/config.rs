//! Service configuration, composed from shared platform config sections.

use serde::Deserialize;

use hrms_platform::config::{DatabaseConfig, KafkaConfig, ServerConfig};

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub kafka: KafkaConfig,
    pub core_banking_endpoint: String,
    pub encryption_key_id: String,
}
