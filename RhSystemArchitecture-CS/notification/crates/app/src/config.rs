//! Service configuration, composed from shared platform config sections.

use serde::Deserialize;

use hrms_platform::config::{DatabaseConfig, KafkaConfig, ServerConfig};

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub kafka: KafkaConfig,
    pub email_gateway_url: String,
    pub sms_gateway_url: String,
    pub push_gateway_url: String,
}
