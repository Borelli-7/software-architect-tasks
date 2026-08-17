//! Generic presentational widgets used by native feature views.

use dioxus::prelude::*;

/// Page heading.
#[component]
pub fn PageHeader(title: String) -> Element {
    rsx! { div { class: "hrms-page-header", h1 { "{title}" } } }
}

/// Placeholder shown while data loads or a screen is not yet implemented.
#[component]
pub fn Loading() -> Element {
    rsx! { div { class: "hrms-loading", "Loading…" } }
}
