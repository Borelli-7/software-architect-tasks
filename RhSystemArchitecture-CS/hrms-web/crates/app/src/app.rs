//! Root application component: meta context, global state, router and shell.

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta, Stylesheet, Title};
use leptos_router::components::Router;

use crate::routes::AppRoutes;
use crate::shell::Shell;
use crate::state::provide_app_state;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_app_state();

    view! {
        <Title text="Bank HRMS" />
        <Meta name="description" content="Bank Human Resource Management System" />
        <Stylesheet id="tokens" href="/tokens.css" />
        <Router>
            <Shell>
                <AppRoutes />
            </Shell>
        </Router>
    }
}
