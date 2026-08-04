//! Central route table wiring every feature module into the router.

use leptos::prelude::*;
use leptos_router::components::{Route, Routes};
use leptos_router::path;

use crate::components::not_found::NotFound;
use crate::features;

#[component]
pub fn AppRoutes() -> impl IntoView {
    view! {
        <Routes fallback=|| view! { <NotFound /> }>
            <Route path=path!("/") view=features::dashboard::Dashboard />

            <Route path=path!("/employees") view=features::employee_admin::EmployeeList />
            <Route path=path!("/employees/:id") view=features::employee_admin::EmployeeDetail />

            <Route path=path!("/evaluations") view=features::performance_eval::EvaluationList />
            <Route path=path!("/evaluations/:id") view=features::performance_eval::EvaluationDetail />

            <Route path=path!("/career-paths") view=features::career_planning::CareerPathList />
            <Route path=path!("/career-paths/:id") view=features::career_planning::CareerPathDetail />

            <Route path=path!("/training") view=features::training::TrainingList />
            <Route path=path!("/training/:id") view=features::training::TrainingDetail />

            <Route path=path!("/payroll") view=features::payroll::PayrollRunList />
            <Route path=path!("/payroll/:id") view=features::payroll::PayrollRunDetail />

            <Route path=path!("/users") view=features::iam::UserList />
            <Route path=path!("/users/:id") view=features::iam::UserDetail />

            <Route path=path!("/notifications") view=features::notifications::NotificationConsole />
        </Routes>
    }
}
