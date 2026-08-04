//! Training program screens.

use dioxus::prelude::*;

use crate::components::widgets::{Loading, PageHeader};

#[component]
pub fn TrainingList() -> Element {
    // TODO: use_resource(|| api().training().list()) and render a table.
    rsx! {
        PageHeader { title: "Training" }
        Loading {}
    }
}

#[component]
pub fn TrainingDetail(id: String) -> Element {
    let _ = id;
    // TODO: use_resource to load api().training().get(id) and render enrollments.
    rsx! {
        PageHeader { title: "Training Program" }
        Loading {}
    }
}
