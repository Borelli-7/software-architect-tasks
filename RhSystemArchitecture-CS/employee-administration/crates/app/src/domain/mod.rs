//! Domain layer: the innermost hexagon. Pure business types and ports with no
//! dependency on Axum, sqlx, or any I/O framework.

pub mod employee;
pub mod errors;
pub mod events;
pub mod ports;
