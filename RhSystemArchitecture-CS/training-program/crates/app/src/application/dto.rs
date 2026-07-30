//! Data-transfer objects returned by training use cases.

use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct ProgramDto {
    pub id: Uuid,
    pub title: String,
    pub capacity: u32,
    pub enrolled_count: usize,
}
