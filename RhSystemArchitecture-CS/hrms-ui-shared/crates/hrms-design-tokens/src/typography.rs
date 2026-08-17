//! Typography tokens: font family and a type scale.

/// A named font-size step and its CSS length.
#[derive(Debug, Clone, Copy)]
pub struct FontSize {
    pub name: &'static str,
    pub rem: &'static str,
}

pub const FONT_FAMILY: &str =
    "Inter, system-ui, -apple-system, Segoe UI, Roboto, Helvetica, Arial, sans-serif";

pub const BODY: FontSize = FontSize { name: "font-body", rem: "1rem" };
pub const CAPTION: FontSize = FontSize { name: "font-caption", rem: "0.875rem" };
pub const H3: FontSize = FontSize { name: "font-h3", rem: "1.25rem" };
pub const H2: FontSize = FontSize { name: "font-h2", rem: "1.5rem" };
pub const H1: FontSize = FontSize { name: "font-h1", rem: "2rem" };

pub const ALL: &[FontSize] = &[CAPTION, BODY, H3, H2, H1];
