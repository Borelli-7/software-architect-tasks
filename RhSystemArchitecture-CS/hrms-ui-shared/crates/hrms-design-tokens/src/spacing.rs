//! Spacing scale (rem-based) shared across layouts.

/// A named spacing step and its CSS length.
#[derive(Debug, Clone, Copy)]
pub struct Space {
    pub name: &'static str,
    pub rem: &'static str,
}

pub const XS: Space = Space { name: "space-xs", rem: "0.25rem" };
pub const SM: Space = Space { name: "space-sm", rem: "0.5rem" };
pub const MD: Space = Space { name: "space-md", rem: "1rem" };
pub const LG: Space = Space { name: "space-lg", rem: "1.5rem" };
pub const XL: Space = Space { name: "space-xl", rem: "2.5rem" };

pub const ALL: &[Space] = &[XS, SM, MD, LG, XL];
