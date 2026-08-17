//! Not-found fallback.

use dioxus::prelude::*;

use crate::components::widgets::PageHeader;

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    let _ = segments;
    rsx! {
        PageHeader { title: "404 — Not found" }
    }
}
