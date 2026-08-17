//! Landing dashboard.

use leptos::prelude::*;

use crate::components::widgets::PageHeader;

#[component]
pub fn Dashboard() -> impl IntoView {
    view! {
        <PageHeader title="Dashboard" />
        <p>"Welcome to the Bank HRMS."</p>
    }
}
