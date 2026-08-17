//! Performance evaluation screens.

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::widgets::{Loading, PageHeader};

#[component]
pub fn EvaluationList() -> impl IntoView {
    // TODO: load via api().evaluations().list() and render a table.
    view! {
        <PageHeader title="Evaluations" />
        <Loading />
    }
}

#[component]
pub fn EvaluationDetail() -> impl IntoView {
    let params = use_params_map();
    let _id = move || params.read().get("id").unwrap_or_default();
    // TODO: load via api().evaluations().get(id) and render the evaluation.
    view! {
        <PageHeader title="Evaluation" />
        <Loading />
    }
}
