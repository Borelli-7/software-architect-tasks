//! 404 fallback view.

use leptos::prelude::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! { <section class="hrms-notfound"><h1>"404 — Not found"</h1></section> }
}
