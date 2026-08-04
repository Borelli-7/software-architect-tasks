//! Identity & access management screens.

use dioxus::prelude::*;

use crate::components::widgets::{Loading, PageHeader};

#[component]
pub fn UserList() -> Element {
    // TODO: use_resource(|| api().iam().list()) and render a table.
    rsx! {
        PageHeader { title: "Users" }
        Loading {}
    }
}

#[component]
pub fn UserDetail(id: String) -> Element {
    let _ = id;
    // TODO: load user; expose assign-role and deactivate actions.
    rsx! {
        PageHeader { title: "User" }
        Loading {}
    }
}
