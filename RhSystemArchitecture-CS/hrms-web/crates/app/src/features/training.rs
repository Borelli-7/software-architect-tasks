//! Training program screens.

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::widgets::{Loading, PageHeader};

#[component]
pub fn TrainingList() -> impl IntoView {
    // TODO: load via api().training().list() and render a table.
    view! {
        <PageHeader title="Training" />
        <Loading />
    }
}

#[component]
pub fn TrainingDetail() -> impl IntoView {
    let params = use_params_map();
    let _id = move || params.read().get("id").unwrap_or_default();
    // TODO: load via api().training().get(id) and render program + enrollments.
    view! {
        <PageHeader title="Training Program" />
        <Loading />
    }
}
