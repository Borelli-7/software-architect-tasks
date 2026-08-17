//! Framework-agnostic design tokens shared by the Leptos and Dioxus frontends.
//!
//! Plain Rust constants plus a CSS custom-property generator, so both renderers
//! consume one source of truth for colour, spacing and typography.
#![allow(dead_code)]

pub mod color;
pub mod spacing;
pub mod typography;

/// Emits all tokens as a `:root { --token: value; }` CSS block.
pub fn css_root_variables() -> String {
    todo!("concatenate color, spacing and typography custom properties into a :root block")
}
