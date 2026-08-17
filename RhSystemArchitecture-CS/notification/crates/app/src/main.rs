//! Notification service entry point.
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

use crate::application::ports::ChannelGateway;
use crate::application::use_cases::{RenderTemplate, SendNotification};
use crate::bootstrap::state::AppState;
use crate::config::AppConfig;
use crate::infrastructure::email_gateway::EmailGateway;
use crate::infrastructure::event_publisher::KafkaNotificationEventPublisher;
use crate::infrastructure::pg_notification_repository::PgNotificationRepository;
use crate::infrastructure::pg_template_repository::PgTemplateRepository;
use crate::infrastructure::push_gateway::PushGateway;
use crate::infrastructure::sms_gateway::SmsGateway;
use crate::presentation::router::notification_router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    hrms_platform::telemetry::init("notification");

    let config: AppConfig = hrms_platform::config::load()?;

    // Data layer.
    let pool = hrms_persistence::connect(&config.database.url, config.database.max_connections).await?;
    sqlx::migrate!().run(&pool).await?;

    // Infrastructure adapters (driven side of the hexagon).
    let notifications = Arc::new(PgNotificationRepository::new(pool.clone()));
    let templates = Arc::new(PgTemplateRepository::new(pool.clone()));
    let gateways: Vec<Arc<dyn ChannelGateway>> = vec![
        Arc::new(EmailGateway::new(config.email_gateway_url.clone())),
        Arc::new(SmsGateway::new(config.sms_gateway_url.clone())),
        Arc::new(PushGateway::new(config.push_gateway_url.clone())),
    ];
    let raw_publisher = Arc::new(InMemoryEventPublisher);
    let events = Arc::new(KafkaNotificationEventPublisher::new(
        raw_publisher,
        notification_contracts::TOPIC.to_string(),
    ));
    let clock = Arc::new(SystemClock);
    let ids = Arc::new(UuidGenerator);

    // Application use cases (driving side), with dependencies injected.
    let send_notification = Arc::new(SendNotification {
        notifications: notifications.clone(),
        gateways,
        events: events.clone(),
        clock: clock.clone(),
        ids: ids.clone(),
    });
    let render_template = Arc::new(RenderTemplate {
        templates: templates.clone(),
    });

    let state = AppState { send_notification, render_template };

    // Presentation layer.
    let app: Router = Router::new()
        .merge(notification_router())
        .merge(hrms_platform::health::routes())
        .with_state(state);

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!(%addr, "notification listening");

    axum::serve(listener, app)
        .with_graceful_shutdown(hrms_platform::shutdown::signal())
        .await?;

    Ok(())
}
