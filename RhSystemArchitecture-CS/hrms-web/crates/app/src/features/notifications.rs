//! Notification console screen.

use leptos::prelude::*;

use crate::components::widgets::{Loading, PageHeader};

#[component]
pub fn NotificationConsole() -> impl IntoView {
    // TODO: compose + send notifications and preview rendered templates.
    view! {
        <PageHeader title="Notifications" />
        <Loading />
    }
}
