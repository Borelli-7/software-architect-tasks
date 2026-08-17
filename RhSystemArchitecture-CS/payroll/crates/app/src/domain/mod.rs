//! Domain layer for payroll: the PayrollRun aggregate, payslips, and ports.
//! Isolated, high-security bounded context — no framework dependencies.

pub mod errors;
pub mod events;
pub mod payroll_run;
pub mod payslip;
pub mod ports;
