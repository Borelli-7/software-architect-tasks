//! Performance Evaluation service entry point.
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

use crate::application::use_cases::{CloseCampaign, OpenCampaign, SetObjectives, SubmitReview};
use crate::bootstrap::state::AppState;
use crate::config::AppConfig;
use crate::infrastructure::analytics::WeightedAnalyticsEngine;
use crate::infrastructure::event_publisher::KafkaEvaluationEventPublisher;
use crate::infrastructure::pg_campaign_repository::PgCampaignRepository;
use crate::infrastructure::pg_objective_repository::PgObjectiveRepository;
use crate::infrastructure::pg_review_repository::PgReviewRepository;
use crate::presentation::router::evaluation_router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    hrms_platform::telemetry::init("performance-evaluation");

    let config: AppConfig = hrms_platform::config::load()?;

    // Data layer.
    let pool = hrms_persistence::connect(&config.database.url, config.database.max_connections).await?;
    sqlx::migrate!().run(&pool).await?;

    // Infrastructure adapters (driven side of the hexagon).
    let campaigns = Arc::new(PgCampaignRepository::new(pool.clone()));
    let reviews = Arc::new(PgReviewRepository::new(pool.clone()));
    let objectives = Arc::new(PgObjectiveRepository::new(pool.clone()));
    let analytics = Arc::new(WeightedAnalyticsEngine::new());
    let raw_publisher = Arc::new(InMemoryEventPublisher);
    let events = Arc::new(KafkaEvaluationEventPublisher::new(
        raw_publisher,
        performance_evaluation_contracts::TOPIC.to_string(),
    ));
    let clock = Arc::new(SystemClock);
    let ids = Arc::new(UuidGenerator);

    // Application use cases (driving side), with dependencies injected.
    let open_campaign = Arc::new(OpenCampaign {
        campaigns: campaigns.clone(),
        events: events.clone(),
        clock: clock.clone(),
        ids: ids.clone(),
    });
    let close_campaign = Arc::new(CloseCampaign {
        campaigns: campaigns.clone(),
        events: events.clone(),
    });
    let submit_review = Arc::new(SubmitReview {
        reviews: reviews.clone(),
        analytics: analytics.clone(),
        events: events.clone(),
    });
    let set_objectives = Arc::new(SetObjectives {
        objectives: objectives.clone(),
        ids: ids.clone(),
    });

    let state = AppState { open_campaign, close_campaign, submit_review, set_objectives };

    // Presentation layer.
    let app: Router = Router::new()
        .merge(evaluation_router())
        .merge(hrms_platform::health::routes())
        .with_state(state);

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!(%addr, "performance-evaluation listening");

    axum::serve(listener, app)
        .with_graceful_shutdown(hrms_platform::shutdown::signal())
        .await?;

    Ok(())
}
