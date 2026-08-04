//! Employee administration screens.

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::widgets::{Loading, PageHeader};

#[component]
pub fn EmployeeList() -> impl IntoView {
    // TODO: load via api().employees().list() into a LocalResource and render a table.
    view! {
        <PageHeader title="Employees" />
        <Loading />
    }
}

#[component]
pub fn EmployeeDetail() -> impl IntoView {
    let params = use_params_map();
    let _id = move || params.read().get("id").unwrap_or_default();
    // TODO: load via api().employees().get(id) and render the employee record.
    view! {
        <PageHeader title="Employee" />
        <Loading />
    }
}
