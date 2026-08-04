//! Identity & access management screens.

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::widgets::{Loading, PageHeader};

#[component]
pub fn UserList() -> impl IntoView {
    // TODO: load via api().iam().list() and render a table.
    view! {
        <PageHeader title="Users" />
        <Loading />
    }
}

#[component]
pub fn UserDetail() -> impl IntoView {
    let params = use_params_map();
    let _id = move || params.read().get("id").unwrap_or_default();
    // TODO: load user; expose assign-role and deactivate actions.
    view! {
        <PageHeader title="User" />
        <Loading />
    }
}
