//! Bank HRMS web client entry point (Leptos CSR).
#![allow(dead_code)]

mod api;
mod app;
mod components;
mod config;
mod features;
mod routes;
mod shell;
mod state;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
