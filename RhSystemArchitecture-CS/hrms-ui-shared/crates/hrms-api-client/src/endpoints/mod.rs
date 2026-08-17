//! Typed endpoint groups, one module per backend service.
//!
//! Each group borrows an [`ApiClient`](crate::ApiClient) and exposes async
//! methods returning DTOs. Request bodies are stubbed with `todo!()` until the
//! transport helper is finalised.

pub mod audit;
pub mod career;
pub mod document;
pub mod employee;
pub mod evaluation;
pub mod iam;
pub mod notification;
pub mod payroll;
pub mod training;

use crate::ApiClient;

impl ApiClient {
    pub fn employees(&self) -> employee::EmployeeApi<'_> {
        employee::EmployeeApi(self)
    }
    pub fn payroll(&self) -> payroll::PayrollApi<'_> {
        payroll::PayrollApi(self)
    }
    pub fn evaluations(&self) -> evaluation::EvaluationApi<'_> {
        evaluation::EvaluationApi(self)
    }
    pub fn career(&self) -> career::CareerApi<'_> {
        career::CareerApi(self)
    }
    pub fn training(&self) -> training::TrainingApi<'_> {
        training::TrainingApi(self)
    }
    pub fn iam(&self) -> iam::IamApi<'_> {
        iam::IamApi(self)
    }
    pub fn notifications(&self) -> notification::NotificationApi<'_> {
        notification::NotificationApi(self)
    }
    pub fn documents(&self) -> document::DocumentApi<'_> {
        document::DocumentApi(self)
    }
    pub fn audit(&self) -> audit::AuditApi<'_> {
        audit::AuditApi(self)
    }
}
