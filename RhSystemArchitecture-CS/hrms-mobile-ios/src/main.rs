//! iOS launcher for the Bank HRMS. Thin shell over the shared UI.

fn main() {
    dioxus::launch(hrms_ui_core::App);
}
