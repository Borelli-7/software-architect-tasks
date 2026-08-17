//! Payroll screens (dual-authorization run lifecycle).

use dioxus::prelude::*;

use crate::components::widgets::{Loading, PageHeader};

#[component]
pub fn PayrollRunList() -> Element {
    // TODO: use_resource(|| api().payroll().list_runs()) and render a table.
    rsx! {
        PageHeader { title: "Payroll Runs" }
        Loading {}
    }
}

#[component]
pub fn PayrollRunDetail(id: String) -> Element {
    let _ = id;
    // TODO: load run + payslips; expose initiate/approve dual-authorization actions.
    rsx! {
        PageHeader { title: "Payroll Run" }
        Loading {}
    }
}
