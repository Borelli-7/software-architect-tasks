//! The TrainingProgram aggregate with enrollments and certifications.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::errors::TrainingError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProgramId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EnrollmentId(pub Uuid);

/// Source of the training program.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProgramProvider {
    Internal,
    External { provider_name: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnrollmentStatus {
    Enrolled,
    InProgress,
    Completed,
    Cancelled,
}

/// A single employee's enrollment in a program.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enrollment {
    pub id: EnrollmentId,
    pub employee_id: Uuid,
    pub status: EnrollmentStatus,
    pub certificate_id: Option<Uuid>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// Aggregate root: a training program with its roster of enrollments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingProgram {
    pub id: ProgramId,
    pub title: String,
    pub provider: ProgramProvider,
    pub capacity: u32,
    pub enrollments: Vec<Enrollment>,
}

impl TrainingProgram {
    /// Enrolls an employee, enforcing capacity and duplicate checks.
    pub fn enroll(&mut self, _employee_id: Uuid, _enrollment_id: EnrollmentId) -> Result<(), TrainingError> {
        todo!("guard capacity and duplicates, append enrollment")
    }

    /// Marks an enrollment complete and attaches a certificate.
    pub fn complete(&mut self, _enrollment_id: EnrollmentId, _at: DateTime<Utc>) -> Result<(), TrainingError> {
        todo!("transition enrollment to Completed and set certificate")
    }
}
