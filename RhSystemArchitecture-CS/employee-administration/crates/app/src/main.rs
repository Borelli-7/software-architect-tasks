//! Employee Administration service entry point.
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

use crate::application::use_cases::{CreateEmployee, TerminateEmployee, UpdateEmployee};
use crate::bootstrap::state::AppState;
use crate::config::AppConfig;
use crate::infrastructure::active_directory::ActiveDirectoryClient;
use crate::infrastructure::event_publisher::KafkaEmployeeEventPublisher;
use crate::infrastructure::pg_employee_repository::PgEmployeeRepository;
use crate::presentation::router::employee_router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    hrms_platform::telemetry::init("employee-administration");

    let config: AppConfig = hrms_platform::config::load()?;

    // Data layer.
    let pool = hrms_persistence::connect(&config.database.url, config.database.max_connections).await?;
    sqlx::migrate!().run(&pool).await?;

    // Infrastructure adapters (driven side of the hexagon).
    let repository = Arc::new(PgEmployeeRepository::new(pool.clone()));
    let raw_publisher = Arc::new(InMemoryEventPublisher);
    let events = Arc::new(KafkaEmployeeEventPublisher::new(
        raw_publisher,
        employee_administration_contracts::TOPIC,
    ));
    let identity = Arc::new(ActiveDirectoryClient::new(config.active_directory_url.clone()));
    let clock = Arc::new(SystemClock);
    let ids = Arc::new(UuidGenerator);

    // Application use cases (driving side), with dependencies injected.
    let create_employee = Arc::new(CreateEmployee {
        repository: repository.clone(),
        events: events.clone(),
        identity: identity.clone(),
        clock: clock.clone(),
        ids: ids.clone(),
    });
    let update_employee = Arc::new(UpdateEmployee { repository: repository.clone() });
    let terminate_employee = Arc::new(TerminateEmployee {
        repository: repository.clone(),
        events: events.clone(),
        identity: identity.clone(),
        clock: clock.clone(),
    });

    let state = AppState { create_employee, update_employee, terminate_employee };

    // Presentation layer.
    let app: Router = Router::new()
        .merge(employee_router())
        .merge(hrms_platform::health::routes())
        .with_state(state);

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!(%addr, "employee-administration listening");

    axum::serve(listener, app)
        .with_graceful_shutdown(hrms_platform::shutdown::signal())
        .await?;

    Ok(())
}
