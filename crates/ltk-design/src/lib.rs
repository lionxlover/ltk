//! # ltk-design
//! Complete design token system for LTK.
//! Covers color, typography, spacing, elevation/glass, icons, motion, and theming.

#![warn(missing_docs)]

pub mod color;
pub mod typography;
pub mod spacing;
pub mod elevation;
pub mod icon;
pub mod motion;
pub mod theme;

// Re-export the most commonly used items
pub use color::{
    ColorToken, ColorPalette, ThemeColors,
    ContrastEngine, AccentGenerator, DynamicColor,
};
pub use typography::{
    FontRegistry, FontSpec, TextStyle, TypeScale,
    FontWeight, FontStyle, TextDecoration,
};
pub use spacing::{SpaceToken, SpaceScale, LayoutTokens};
pub use elevation::{ShadowSpec, BlurSpec, GlassSpec, ElevationLevel};
pub use icon::{IconName, IconRegistry, IconTheme};
pub use motion::{Easing, DurationToken, MotionTokens};
pub use theme::{Theme, ThemeEngine, ThemeMode, ThemeChangeEvent};
