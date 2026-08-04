//! Performance evaluation screens.

use dioxus::prelude::*;

use crate::components::widgets::{Loading, PageHeader};

#[component]
pub fn EvaluationList() -> Element {
    // TODO: use_resource(|| api().evaluations().list()) and render a table.
    rsx! {
        PageHeader { title: "Evaluations" }
        Loading {}
    }
}

#[component]
pub fn EvaluationDetail(id: String) -> Element {
    let _ = id;
    // TODO: use_resource to load api().evaluations().get(id).
    rsx! {
        PageHeader { title: "Evaluation" }
        Loading {}
    }
}
