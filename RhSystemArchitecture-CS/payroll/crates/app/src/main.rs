//! Payroll service entry point.
//!
//! Composition root wiring the hexagonal layers together: load config, init
//! telemetry, build the DB pool, run migrations, construct infrastructure
//! adapters (repository, tax engine, salary encryptor, core-banking saga,
//! event publisher), inject them into application use cases, mount the HTTP
//! router and health probes, then serve with graceful shutdown.
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

use crate::application::use_cases::{ApprovePayroll, RunPayroll, SubmitPaymentBatch};
use crate::bootstrap::state::AppState;
use crate::config::AppConfig;
use crate::infrastructure::core_banking_saga::CoreBankingSaga;
use crate::infrastructure::event_publisher::KafkaPayrollEventPublisher;
use crate::infrastructure::field_encryptor::HsmSalaryEncryptor;
use crate::infrastructure::pg_payroll_repository::PgPayrollRepository;
use crate::infrastructure::tax_engine::RuleBasedTaxEngine;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    hrms_platform::telemetry::init("payroll");

    let config: AppConfig = hrms_platform::config::load()?;

    // Data layer.
    let pool = hrms_persistence::connect(&config.database.url, config.database.max_connections).await?;
    sqlx::migrate!().run(&pool).await?;

    // Infrastructure adapters (driven side of the hexagon).
    let repository = Arc::new(PgPayrollRepository::new(pool.clone()));
    let tax_engine = Arc::new(RuleBasedTaxEngine::new());
    let raw_publisher = Arc::new(InMemoryEventPublisher);
    let events = Arc::new(KafkaPayrollEventPublisher::new(
        raw_publisher,
        payroll_contracts::TOPIC.to_string(),
    ));
    let saga = Arc::new(CoreBankingSaga::new(config.core_banking_endpoint.clone()));
    // Field-level salary encryptor; injected into repository once queries land.
    let _encryptor = Arc::new(HsmSalaryEncryptor::new(config.encryption_key_id.clone()));
    let clock = Arc::new(SystemClock);
    let ids = Arc::new(UuidGenerator);

    // Application use cases (driving side), with dependencies injected.
    let run_payroll = Arc::new(RunPayroll {
        repository: repository.clone(),
        tax_engine: tax_engine.clone(),
        clock: clock.clone(),
        ids: ids.clone(),
    });
    let approve_payroll = Arc::new(ApprovePayroll {
        repository: repository.clone(),
        events: events.clone(),
        clock: clock.clone(),
    });
    let submit_payment_batch = Arc::new(SubmitPaymentBatch {
        repository: repository.clone(),
        saga: saga.clone(),
        events: events.clone(),
    });

    let state = AppState { run_payroll, approve_payroll, submit_payment_batch };

    // Presentation layer.
    let app: Router = Router::new()
        .merge(presentation::router::payroll_router())
        .merge(hrms_platform::health::routes())
        .with_state(state);

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!(%addr, "payroll listening");

    axum::serve(listener, app)
        .with_graceful_shutdown(hrms_platform::shutdown::signal())
        .await?;

    Ok(())
}
