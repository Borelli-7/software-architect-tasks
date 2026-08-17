//! Identity & Access Management service entry point.
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

use crate::application::use_cases::{AssignRole, CreateUser, DeactivateUser};
use crate::bootstrap::state::AppState;
use crate::config::AppConfig;
use crate::infrastructure::event_publisher::KafkaIamEventPublisher;
use crate::infrastructure::keycloak_federation::KeycloakFederation;
use crate::infrastructure::pg_user_repository::PgUserRepository;
use crate::presentation::router::iam_router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    hrms_platform::telemetry::init("iam");

    let config: AppConfig = hrms_platform::config::load()?;

    // Data layer.
    let pool = hrms_persistence::connect(&config.database.url, config.database.max_connections).await?;
    sqlx::migrate!().run(&pool).await?;

    // Infrastructure adapters (driven side of the hexagon).
    let users = Arc::new(PgUserRepository::new(pool.clone()));
    let directory = Arc::new(KeycloakFederation::new(
        config.identity_provider_url.clone(),
        config.identity_provider_realm.clone(),
    ));
    let raw_publisher = Arc::new(InMemoryEventPublisher);
    let events = Arc::new(KafkaIamEventPublisher::new(
        raw_publisher,
        iam_contracts::TOPIC.to_string(),
    ));
    let clock = Arc::new(SystemClock);
    let ids = Arc::new(UuidGenerator);

    // Application use cases (driving side), with dependencies injected.
    let create_user = Arc::new(CreateUser {
        users: users.clone(),
        directory: directory.clone(),
        events: events.clone(),
        clock: clock.clone(),
        ids: ids.clone(),
    });
    let assign_role = Arc::new(AssignRole {
        users: users.clone(),
        events: events.clone(),
    });
    let deactivate_user = Arc::new(DeactivateUser {
        users: users.clone(),
        directory: directory.clone(),
        events: events.clone(),
    });

    let state = AppState { create_user, assign_role, deactivate_user };

    // Presentation layer.
    let app: Router = Router::new()
        .merge(iam_router())
        .merge(hrms_platform::health::routes())
        .with_state(state);

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!(%addr, "iam listening");

    axum::serve(listener, app)
        .with_graceful_shutdown(hrms_platform::shutdown::signal())
        .await?;

    Ok(())
}
