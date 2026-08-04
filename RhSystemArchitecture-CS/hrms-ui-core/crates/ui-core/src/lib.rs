//! Shared Dioxus UI for the Bank HRMS native apps (Windows, Linux, iOS, Android).
//!
//! Every launcher calls [`App`]; platform selection happens through the cargo
//! feature flags (`web` / `desktop` / `mobile`) forwarded to `dioxus`.
#![allow(dead_code)]

mod app;
mod api;
mod components;
mod config;
mod features;
mod routes;
mod shell;
mod state;

pub use app::App;
