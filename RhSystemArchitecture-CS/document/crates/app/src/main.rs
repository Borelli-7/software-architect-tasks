//! Document service entry point.
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

use crate::application::use_cases::{GenerateDocument, RetrieveDocument, UploadDocument};
use crate::bootstrap::state::AppState;
use crate::config::AppConfig;
use crate::infrastructure::blob_store::S3BlobStore;
use crate::infrastructure::document_generator::HttpDocumentGenerator;
use crate::infrastructure::event_publisher::KafkaDocumentEventPublisher;
use crate::infrastructure::pg_document_repository::PgDocumentRepository;
use crate::presentation::router::document_router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    hrms_platform::telemetry::init("document");

    let config: AppConfig = hrms_platform::config::load()?;

    // Data layer.
    let pool = hrms_persistence::connect(&config.database.url, config.database.max_connections).await?;
    sqlx::migrate!().run(&pool).await?;

    // Infrastructure adapters (driven side of the hexagon).
    let documents = Arc::new(PgDocumentRepository::new(pool.clone()));
    let blobs = Arc::new(S3BlobStore::new(
        config.blob_bucket.clone(),
        config.blob_endpoint.clone(),
    ));
    let generator = Arc::new(HttpDocumentGenerator::new(config.document_generator_url.clone()));
    let raw_publisher = Arc::new(InMemoryEventPublisher);
    let events = Arc::new(KafkaDocumentEventPublisher::new(
        raw_publisher,
        document_contracts::TOPIC.to_string(),
    ));
    let clock = Arc::new(SystemClock);
    let ids = Arc::new(UuidGenerator);

    // Application use cases (driving side), with dependencies injected.
    let upload_document = Arc::new(UploadDocument {
        documents: documents.clone(),
        blobs: blobs.clone(),
        events: events.clone(),
        clock: clock.clone(),
        ids: ids.clone(),
    });
    let generate_document = Arc::new(GenerateDocument {
        documents: documents.clone(),
        blobs: blobs.clone(),
        generator: generator.clone(),
        events: events.clone(),
        clock: clock.clone(),
        ids: ids.clone(),
    });
    let retrieve_document = Arc::new(RetrieveDocument {
        documents: documents.clone(),
        blobs: blobs.clone(),
    });

    let state = AppState { upload_document, generate_document, retrieve_document };

    // Presentation layer.
    let app: Router = Router::new()
        .merge(document_router())
        .merge(hrms_platform::health::routes())
        .with_state(state);

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!(%addr, "document listening");

    axum::serve(listener, app)
        .with_graceful_shutdown(hrms_platform::shutdown::signal())
        .await?;

    Ok(())
}
