//! Use case implementations, wiring the domain to injected outbound ports.

use std::sync::Arc;

use async_trait::async_trait;
use hrms_kernel::{Clock, IdGenerator};

use super::CommandHandler;
use super::commands::{ApprovePayrollCommand, RunPayrollCommand, SubmitPaymentBatchCommand};
use super::dto::PayrollRunDto;
use super::errors::ApplicationError;
use super::ports::{PaymentSaga, PayrollEventPublisher};
use crate::domain::ports::{PayrollRepository, TaxEngine};

#[derive(Clone)]
pub struct RunPayroll {
    pub repository: Arc<dyn PayrollRepository>,
    pub tax_engine: Arc<dyn TaxEngine>,
    pub clock: Arc<dyn Clock>,
    pub ids: Arc<dyn IdGenerator>,
}

#[async_trait]
impl CommandHandler<RunPayrollCommand> for RunPayroll {
    type Output = PayrollRunDto;

    async fn handle(&self, _command: RunPayrollCommand) -> Result<Self::Output, ApplicationError> {
        todo!("gather employees + salary structures, compute gross/net via tax engine, persist Draft->Calculated")
    }
}

#[derive(Clone)]
pub struct ApprovePayroll {
    pub repository: Arc<dyn PayrollRepository>,
    pub events: Arc<dyn PayrollEventPublisher>,
    pub clock: Arc<dyn Clock>,
}

#[async_trait]
impl CommandHandler<ApprovePayrollCommand> for ApprovePayroll {
    type Output = PayrollRunDto;

    async fn handle(&self, _command: ApprovePayrollCommand) -> Result<Self::Output, ApplicationError> {
        todo!("apply approval with dual-control rules; publish PayrollApproved when fully signed off")
    }
}

#[derive(Clone)]
pub struct SubmitPaymentBatch {
    pub repository: Arc<dyn PayrollRepository>,
    pub saga: Arc<dyn PaymentSaga>,
    pub events: Arc<dyn PayrollEventPublisher>,
}

#[async_trait]
impl CommandHandler<SubmitPaymentBatchCommand> for SubmitPaymentBatch {
    type Output = ();

    async fn handle(&self, _command: SubmitPaymentBatchCommand) -> Result<Self::Output, ApplicationError> {
        todo!("guard Approved run, run payment saga against core banking, publish PayrollPaid or compensate")
    }
}
