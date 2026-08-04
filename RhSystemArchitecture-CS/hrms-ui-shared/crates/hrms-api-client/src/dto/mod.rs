//! Data-transfer objects mirroring the backend service contracts.
//!
//! These are the wire shapes exchanged with the API gateway. They intentionally
//! duplicate the backend `*-contracts` crates so the frontend has no build-time
//! dependency on server code.

pub mod audit;
pub mod career;
pub mod document;
pub mod employee;
pub mod evaluation;
pub mod iam;
pub mod notification;
pub mod payroll;
pub mod training;
