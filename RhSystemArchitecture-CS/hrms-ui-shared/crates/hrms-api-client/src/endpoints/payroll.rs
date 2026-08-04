//! Payroll endpoints (`/payroll-runs`).

use uuid::Uuid;

use crate::ApiClient;
use crate::dto::payroll::{ApprovePayrollRunRequest, InitiatePayrollRunRequest, PayrollRunDto, PayslipDto};
use crate::error::ApiError;

pub struct PayrollApi<'a>(pub(crate) &'a ApiClient);

impl PayrollApi<'_> {
    pub async fn list_runs(&self) -> Result<Vec<PayrollRunDto>, ApiError> {
        todo!("GET /payroll-runs")
    }

    pub async fn get_run(&self, _id: Uuid) -> Result<PayrollRunDto, ApiError> {
        todo!("GET /payroll-runs/{{id}}")
    }

    pub async fn initiate(&self, _req: InitiatePayrollRunRequest) -> Result<PayrollRunDto, ApiError> {
        todo!("POST /payroll-runs")
    }

    pub async fn approve(&self, _id: Uuid, _req: ApprovePayrollRunRequest) -> Result<PayrollRunDto, ApiError> {
        todo!("POST /payroll-runs/{{id}}/approve")
    }

    pub async fn payslips(&self, _run_id: Uuid) -> Result<Vec<PayslipDto>, ApiError> {
        todo!("GET /payroll-runs/{{id}}/payslips")
    }
}
