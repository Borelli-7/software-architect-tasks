//! Infrastructure adapters implementing the domain and application ports.

pub mod email_gateway;
pub mod event_publisher;
pub mod pg_notification_repository;
pub mod pg_template_repository;
pub mod push_gateway;
pub mod sms_gateway;
