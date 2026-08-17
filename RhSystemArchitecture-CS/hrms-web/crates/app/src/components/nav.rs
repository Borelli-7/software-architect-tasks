//! Primary side navigation.

use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn SideNav() -> impl IntoView {
    view! {
        <nav class="hrms-sidenav">
            <A href="/employees">"Employees"</A>
            <A href="/evaluations">"Evaluations"</A>
            <A href="/career-paths">"Career Paths"</A>
            <A href="/training">"Training"</A>
            <A href="/payroll">"Payroll"</A>
            <A href="/users">"Users"</A>
            <A href="/notifications">"Notifications"</A>
        </nav>
    }
}
