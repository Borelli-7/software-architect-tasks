//! Employee administration screens.

use dioxus::prelude::*;

use crate::components::widgets::{Loading, PageHeader};

#[component]
pub fn EmployeeList() -> Element {
    // TODO: use_resource(|| api().employees().list()) and render a table.
    rsx! {
        PageHeader { title: "Employees" }
        Loading {}
    }
}

#[component]
pub fn EmployeeDetail(id: String) -> Element {
    let _ = id;
    // TODO: use_resource to load api().employees().get(id) and render the record.
    rsx! {
        PageHeader { title: "Employee" }
        Loading {}
    }
}
