//! Landing dashboard.

use dioxus::prelude::*;

use crate::components::widgets::PageHeader;

#[component]
pub fn Dashboard() -> Element {
    rsx! {
        PageHeader { title: "Dashboard" }
        p { "Welcome to the Bank HRMS." }
    }
}
