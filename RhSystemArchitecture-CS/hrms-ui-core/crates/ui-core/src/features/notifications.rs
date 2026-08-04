//! Notification console screen.

use dioxus::prelude::*;

use crate::components::widgets::{Loading, PageHeader};

#[component]
pub fn NotificationConsole() -> Element {
    // TODO: compose + send notifications and preview rendered templates.
    rsx! {
        PageHeader { title: "Notifications" }
        Loading {}
    }
}
