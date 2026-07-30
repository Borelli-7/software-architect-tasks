//! Domain events emitted by the payroll aggregate.

use serde::{Deserialize, Serialize};

use super::payroll_run::PayrollRunId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PayrollEvent {
    Calculated { id: PayrollRunId },
    Approved { id: PayrollRunId },
    Submitted { id: PayrollRunId },
    Paid { id: PayrollRunId },
    Failed { id: PayrollRunId },
}
