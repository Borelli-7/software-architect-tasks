//! Payroll screens (dual-authorization run lifecycle).

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::widgets::{Loading, PageHeader};

#[component]
pub fn PayrollRunList() -> impl IntoView {
    // TODO: load via api().payroll().list_runs() and render a table.
    view! {
        <PageHeader title="Payroll Runs" />
        <Loading />
    }
}

#[component]
pub fn PayrollRunDetail() -> impl IntoView {
    let params = use_params_map();
    let _id = move || params.read().get("id").unwrap_or_default();
    // TODO: load run + payslips; expose initiate/approve dual-authorization actions.
    view! {
        <PageHeader title="Payroll Run" />
        <Loading />
    }
}
