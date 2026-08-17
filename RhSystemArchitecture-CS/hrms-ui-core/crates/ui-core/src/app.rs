//! Root component: provides state and mounts the router.

use dioxus::prelude::*;

use crate::routes::Route;
use crate::state::provide_app_state;

#[component]
pub fn App() -> Element {
    provide_app_state();
    rsx! {
        Router::<Route> {}
    }
}
