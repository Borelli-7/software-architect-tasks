//! Route table for the native apps. Each variant maps to a feature component;
//! the [`Shell`](crate::shell::Shell) layout wraps them with navigation.

use dioxus::prelude::*;

use crate::features;
use crate::shell::Shell;

#[rustfmt::skip]
#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[layout(Shell)]
        #[route("/")]
        Dashboard {},

        #[route("/employees")]
        EmployeeList {},
        #[route("/employees/:id")]
        EmployeeDetail { id: String },

        #[route("/evaluations")]
        EvaluationList {},
        #[route("/evaluations/:id")]
        EvaluationDetail { id: String },

        #[route("/career-paths")]
        CareerPathList {},
        #[route("/career-paths/:id")]
        CareerPathDetail { id: String },

        #[route("/training")]
        TrainingList {},
        #[route("/training/:id")]
        TrainingDetail { id: String },

        #[route("/payroll")]
        PayrollRunList {},
        #[route("/payroll/:id")]
        PayrollRunDetail { id: String },

        #[route("/users")]
        UserList {},
        #[route("/users/:id")]
        UserDetail { id: String },

        #[route("/notifications")]
        NotificationConsole {},
    #[end_layout]
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

// Re-export feature components at crate root so the `Routable` derive resolves
// each route variant to a same-named component.
pub use features::career_planning::{CareerPathDetail, CareerPathList};
pub use features::dashboard::Dashboard;
pub use features::employee_admin::{EmployeeDetail, EmployeeList};
pub use features::iam::{UserDetail, UserList};
pub use features::not_found::NotFound;
pub use features::notifications::NotificationConsole;
pub use features::payroll::{PayrollRunDetail, PayrollRunList};
pub use features::performance_eval::{EvaluationDetail, EvaluationList};
pub use features::training::{TrainingDetail, TrainingList};
