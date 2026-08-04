//! Application shell: top bar, side navigation and the routed content outlet.

use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::nav::SideNav;

#[component]
pub fn Shell(children: Children) -> impl IntoView {
    view! {
        <div class="hrms-shell">
            <header class="hrms-topbar">
                <A href="/">"Bank HRMS"</A>
            </header>
            <div class="hrms-body">
                <SideNav />
                <main class="hrms-content">{children()}</main>
            </div>
        </div>
    }
}
