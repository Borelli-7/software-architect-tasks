//! Training Program service entry point.
//!
//! Composition root wiring the hexagonal layers together: load config, init
//! telemetry, build the DB pool, run migrations, construct infrastructure
//! adapters, inject them into application use cases, mount the HTTP router and
//! health probes, then serve with graceful shutdown.
//!
//! Business logic lives behind `todo!()` stubs; the wiring below is complete.

#![allow(dead_code)]

mod application;
mod bootstrap;
mod config;
mod domain;
mod infrastructure;
mod presentation;

use std::sync::Arc;

use axum::Router;
use hrms_kernel::{SystemClock, UuidGenerator};
use hrms_messaging::InMemoryEventPublisher;

use crate::application::use_cases::{CompleteTraining, CreateProgram, EnrollEmployee};
use crate::bootstrap::state::AppState;
use crate::config::AppConfig;
use crate::infrastructure::event_publisher::KafkaTrainingEventPublisher;
use crate::infrastructure::external_provider::HttpProviderClient;
use crate::infrastructure::pg_training_repository::PgTrainingRepository;
use crate::presentation::router::training_router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    hrms_platform::telemetry::init("training-program");

    let config: AppConfig = hrms_platform::config::load()?;

    // Data layer.
    let pool = hrms_persistence::connect(&config.database.url, config.database.max_connections).await?;
    sqlx::migrate!().run(&pool).await?;

    // Infrastructure adapters (driven side of the hexagon).
    let programs = Arc::new(PgTrainingRepository::new(pool.clone()));
    let provider = Arc::new(HttpProviderClient::new(config.external_provider_url.clone()));
    let raw_publisher = Arc::new(InMemoryEventPublisher);
    let events = Arc::new(KafkaTrainingEventPublisher::new(
        raw_publisher,
        training_program_contracts::TOPIC.to_string(),
    ));
    let clock = Arc::new(SystemClock);
    let ids = Arc::new(UuidGenerator);

    // Application use cases (driving side), with dependencies injected.
    let create_program = Arc::new(CreateProgram {
        programs: programs.clone(),
        ids: ids.clone(),
    });
    let enroll_employee = Arc::new(EnrollEmployee {
        programs: programs.clone(),
        provider: provider.clone(),
        events: events.clone(),
        ids: ids.clone(),
    });
    let complete_training = Arc::new(CompleteTraining {
        programs: programs.clone(),
        events: events.clone(),
        clock: clock.clone(),
    });

    let state = AppState { create_program, enroll_employee, complete_training };

    // Presentation layer.
    let app: Router = Router::new()
        .merge(training_router())
        .merge(hrms_platform::health::routes())
        .with_state(state);

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!(%addr, "training-program listening");

    axum::serve(listener, app)
        .with_graceful_shutdown(hrms_platform::shutdown::signal())
        .await?;

    Ok(())
}
