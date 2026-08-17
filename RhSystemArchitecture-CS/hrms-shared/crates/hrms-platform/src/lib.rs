//! Cross-cutting platform utilities shared by every service binary:
//! configuration loading, telemetry, health probes, graceful shutdown, and a
//! transport-level error type.

/// Layered configuration loading (defaults -> file -> environment).
pub mod config {
    use serde::Deserialize;

    #[derive(Debug, Clone, Deserialize)]
    pub struct ServerConfig {
        pub host: String,
        pub port: u16,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct DatabaseConfig {
        pub url: String,
        pub max_connections: u32,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct KafkaConfig {
        pub brokers: String,
        pub group_id: String,
    }

    /// Deserializes a service configuration struct from environment variables
    /// using `__` as the nesting separator (e.g. `SERVER__PORT`).
    pub fn load<T: serde::de::DeserializeOwned>() -> Result<T, ::config::ConfigError> {
        ::config::Config::builder()
            .add_source(::config::Environment::default().separator("__"))
            .build()?
            .try_deserialize()
    }
}

/// Structured JSON tracing initialization.
pub mod telemetry {
    use tracing_subscriber::{EnvFilter, fmt, prelude::*};

    /// Installs a JSON tracing subscriber filtered by `RUST_LOG` (default `info`).
    pub fn init(_service_name: &str) {
        let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt::layer().json())
            .init();
    }
}

/// Kubernetes-style liveness/readiness endpoints.
pub mod health {
    use axum::{Router, http::StatusCode, routing::get};

    pub fn routes<S: Clone + Send + Sync + 'static>() -> Router<S> {
        Router::new()
            .route("/health/live", get(|| async { StatusCode::OK }))
            .route("/health/ready", get(|| async { StatusCode::OK }))
    }
}

/// Graceful shutdown signalling.
pub mod shutdown {
    /// Resolves when the process receives Ctrl-C / SIGINT.
    pub async fn signal() {
        let _ = tokio::signal::ctrl_c().await;
    }
}

/// HTTP-facing application error mapped to problem responses.
pub mod error {
    use axum::{
        Json,
        http::StatusCode,
        response::{IntoResponse, Response},
    };
    use serde::Serialize;

    #[derive(Debug, thiserror::Error)]
    pub enum AppError {
        #[error("bad request: {0}")]
        BadRequest(String),
        #[error("unauthorized")]
        Unauthorized,
        #[error("forbidden")]
        Forbidden,
        #[error("not found: {0}")]
        NotFound(String),
        #[error("conflict: {0}")]
        Conflict(String),
        #[error(transparent)]
        Internal(#[from] anyhow::Error),
    }

    #[derive(Serialize)]
    struct ErrorBody {
        error: String,
    }

    impl IntoResponse for AppError {
        fn into_response(self) -> Response {
            let status = match &self {
                AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
                AppError::Unauthorized => StatusCode::UNAUTHORIZED,
                AppError::Forbidden => StatusCode::FORBIDDEN,
                AppError::NotFound(_) => StatusCode::NOT_FOUND,
                AppError::Conflict(_) => StatusCode::CONFLICT,
                AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (status, Json(ErrorBody { error: self.to_string() })).into_response()
        }
    }
}
