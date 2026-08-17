//! Career path planning screens.

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::widgets::{Loading, PageHeader};

#[component]
pub fn CareerPathList() -> impl IntoView {
    // TODO: load via api().career().list() and render a table.
    view! {
        <PageHeader title="Career Paths" />
        <Loading />
    }
}

#[component]
pub fn CareerPathDetail() -> impl IntoView {
    let params = use_params_map();
    let _id = move || params.read().get("id").unwrap_or_default();
    // TODO: load via api().career().get(id) and render milestones.
    view! {
        <PageHeader title="Career Path" />
        <Loading />
    }
}
