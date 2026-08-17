//! Career path planning screens.

use dioxus::prelude::*;

use crate::components::widgets::{Loading, PageHeader};

#[component]
pub fn CareerPathList() -> Element {
    // TODO: use_resource(|| api().career().list()) and render a table.
    rsx! {
        PageHeader { title: "Career Paths" }
        Loading {}
    }
}

#[component]
pub fn CareerPathDetail(id: String) -> Element {
    let _ = id;
    // TODO: use_resource to load api().career().get(id) and render milestones.
    rsx! {
        PageHeader { title: "Career Path" }
        Loading {}
    }
}
