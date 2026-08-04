//! Primary side navigation for the native apps.

use dioxus::prelude::*;

use crate::routes::Route;

#[component]
pub fn SideNav() -> Element {
    rsx! {
        nav { class: "hrms-sidenav",
            Link { to: Route::EmployeeList {}, "Employees" }
            Link { to: Route::EvaluationList {}, "Evaluations" }
            Link { to: Route::CareerPathList {}, "Career Paths" }
            Link { to: Route::TrainingList {}, "Training" }
            Link { to: Route::PayrollRunList {}, "Payroll" }
            Link { to: Route::UserList {}, "Users" }
            Link { to: Route::NotificationConsole {}, "Notifications" }
        }
    }
}
