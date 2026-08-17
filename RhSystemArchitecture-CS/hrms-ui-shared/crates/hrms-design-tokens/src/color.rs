//! Brand colour palette as hex string tokens.

/// A named colour token and its hex value.
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub name: &'static str,
    pub hex: &'static str,
}

pub const PRIMARY: Color = Color { name: "primary", hex: "#0B5FFF" };
pub const PRIMARY_DARK: Color = Color { name: "primary-dark", hex: "#0842B0" };
pub const SURFACE: Color = Color { name: "surface", hex: "#FFFFFF" };
pub const BACKGROUND: Color = Color { name: "background", hex: "#F4F6FB" };
pub const TEXT: Color = Color { name: "text", hex: "#111827" };
pub const TEXT_MUTED: Color = Color { name: "text-muted", hex: "#6B7280" };
pub const SUCCESS: Color = Color { name: "success", hex: "#12805C" };
pub const WARNING: Color = Color { name: "warning", hex: "#B45309" };
pub const DANGER: Color = Color { name: "danger", hex: "#B42318" };

/// All palette entries, in declaration order.
pub const ALL: &[Color] = &[
    PRIMARY, PRIMARY_DARK, SURFACE, BACKGROUND, TEXT, TEXT_MUTED, SUCCESS, WARNING, DANGER,
];
