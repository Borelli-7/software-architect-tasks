//! Audit Logging service entry point.
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

use crate::application::use_cases::{QueryAuditLog, RecordAuditEntry};
use crate::bootstrap::state::AppState;
use crate::config::AppConfig;
use crate::infrastructure::event_publisher::KafkaAuditEventPublisher;
use crate::infrastructure::pg_audit_repository::PgAuditRepository;
use crate::presentation::router::audit_router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    hrms_platform::telemetry::init("audit-logging");

    let config: AppConfig = hrms_platform::config::load()?;

    // Data layer.
    let pool = hrms_persistence::connect(&config.database.url, config.database.max_connections).await?;
    sqlx::migrate!().run(&pool).await?;

    // Infrastructure adapters (driven side of the hexagon).
    let entries = Arc::new(PgAuditRepository::new(pool.clone()));
    let raw_publisher = Arc::new(InMemoryEventPublisher);
    let events = Arc::new(KafkaAuditEventPublisher::new(
        raw_publisher,
        audit_logging_contracts::TOPIC.to_string(),
    ));
    let clock = Arc::new(SystemClock);
    let ids = Arc::new(UuidGenerator);

    // Application use cases (driving side), with dependencies injected.
    let record_entry = Arc::new(RecordAuditEntry {
        entries: entries.clone(),
        events: events.clone(),
        clock: clock.clone(),
        ids: ids.clone(),
    });
    let query_log = Arc::new(QueryAuditLog {
        entries: entries.clone(),
    });

    let state = AppState { record_entry, query_log };

    // Presentation layer.
    let app: Router = Router::new()
        .merge(audit_router())
        .merge(hrms_platform::health::routes())
        .with_state(state);

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!(%addr, "audit-logging listening");

    axum::serve(listener, app)
        .with_graceful_shutdown(hrms_platform::shutdown::signal())
        .await?;

    Ok(())
}
