//! Generic presentational widgets (cards, tables, buttons) used by feature views.

use leptos::prelude::*;

/// Page heading with an optional action slot.
#[component]
pub fn PageHeader(title: &'static str) -> impl IntoView {
    view! { <div class="hrms-page-header"><h1>{title}</h1></div> }
}

/// Simple card container.
#[component]
pub fn Card(children: Children) -> impl IntoView {
    view! { <div class="hrms-card">{children()}</div> }
}

/// Placeholder shown while data loads or a screen is not yet implemented.
#[component]
pub fn Loading() -> impl IntoView {
    view! { <div class="hrms-loading">"Loading…"</div> }
}
