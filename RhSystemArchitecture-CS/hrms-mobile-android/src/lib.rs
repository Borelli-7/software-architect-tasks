//! Android launcher for the Bank HRMS. Thin shell over the shared UI.
//!
//! The `dx` CLI generates the Gradle/JNI harness that loads this cdylib and
//! invokes the shared entry point below.

/// Shared entry point invoked by the generated Android harness.
pub fn app() {
    dioxus::launch(hrms_ui_core::App);
}
