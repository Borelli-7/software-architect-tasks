//! Application shell layout: navigation plus the routed `Outlet`.

use dioxus::prelude::*;

use crate::components::nav::SideNav;
use crate::routes::Route;

#[component]
pub fn Shell() -> Element {
    rsx! {
        div { class: "hrms-shell",
            header { class: "hrms-topbar",
                Link { to: Route::Dashboard {}, "Bank HRMS" }
            }
            div { class: "hrms-body",
                SideNav {}
                main { class: "hrms-content",
                    Outlet::<Route> {}
                }
            }
        }
    }
}
