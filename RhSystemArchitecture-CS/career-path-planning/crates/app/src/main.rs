//! Career Path Planning service entry point.
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
use hrms_kernel::UuidGenerator;
use hrms_messaging::InMemoryEventPublisher;

use crate::application::use_cases::{DefineCareerPath, RunGapAnalysis};
use crate::bootstrap::state::AppState;
use crate::config::AppConfig;
use crate::infrastructure::event_publisher::KafkaCareerEventPublisher;
use crate::infrastructure::gap_analyzer::LevelGapAnalyzer;
use crate::infrastructure::pg_career_path_repository::PgCareerPathRepository;
use crate::infrastructure::pg_competency_repository::PgCompetencyRepository;
use crate::presentation::router::career_router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    hrms_platform::telemetry::init("career-path-planning");

    let config: AppConfig = hrms_platform::config::load()?;

    // Data layer.
    let pool = hrms_persistence::connect(&config.database.url, config.database.max_connections).await?;
    sqlx::migrate!().run(&pool).await?;

    // Infrastructure adapters (driven side of the hexagon).
    let paths = Arc::new(PgCareerPathRepository::new(pool.clone()));
    let competencies = Arc::new(PgCompetencyRepository::new(pool.clone()));
    let analyzer = Arc::new(LevelGapAnalyzer::new());
    let raw_publisher = Arc::new(InMemoryEventPublisher);
    let events = Arc::new(KafkaCareerEventPublisher::new(
        raw_publisher,
        career_path_planning_contracts::TOPIC.to_string(),
    ));
    let ids = Arc::new(UuidGenerator);

    // Application use cases (driving side), with dependencies injected.
    let define_career_path = Arc::new(DefineCareerPath {
        paths: paths.clone(),
        events: events.clone(),
        ids: ids.clone(),
    });
    let run_gap_analysis = Arc::new(RunGapAnalysis {
        paths: paths.clone(),
        competencies: competencies.clone(),
        analyzer: analyzer.clone(),
        events: events.clone(),
    });

    let state = AppState { define_career_path, run_gap_analysis };

    // Presentation layer.
    let app: Router = Router::new()
        .merge(career_router())
        .merge(hrms_platform::health::routes())
        .with_state(state);

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!(%addr, "career-path-planning listening");

    axum::serve(listener, app)
        .with_graceful_shutdown(hrms_platform::shutdown::signal())
        .await?;

    Ok(())
}
