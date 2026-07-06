//! Color token system with OKLCH-based palette generation and contrast checking.

use ltk_core::Color;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

pub mod tokens;
pub mod generator;
pub mod contrast;
pub mod dynamic;

pub use tokens::*;
pub use generator::AccentGenerator;
pub use contrast::ContrastEngine;
pub use dynamic::DynamicColor;

// ── ColorToken ───────────────────────────────────────────────────────

/// Every named color slot in the design token system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ColorToken {
    // Surfaces
    BgBase, BgSurface, BgSurfaceRaised, BgOverlay,
    // Accents
    Primary, PrimaryDim, PrimaryGlow, PrimaryContainer, OnPrimary,
    AccentPurple, AccentGreen, AccentAmber, AccentRed,
    // Text
    TextPrimary, TextSecondary, TextTertiary, TextDisabled,
    TextOnPrimary, TextOnSurface,
    // Borders
    BorderSubtle, BorderBase, BorderStrong, BorderFocus, BorderError,
    // State
    StateSuccess, StateSuccessBg,
    StateWarning, StateWarningBg,
    StateError,   StateErrorBg,
    StateInfo,    StateInfoBg,
    // Shadow / Scrim
    ShadowSm, ShadowMd, ShadowLg, ShadowXl,
    Scrim,
}

/// A resolved color palette: all tokens mapped to concrete `Color` values.
#[derive(Debug, Clone)]
pub struct ColorPalette(pub HashMap<ColorToken, Color>);

impl ColorPalette {
    pub fn get(&self, token: ColorToken) -> Color {
        *self.0.get(&token).unwrap_or(&Color::BLACK)
    }

    pub fn set(&mut self, token: ColorToken, color: Color) {
        self.0.insert(token, color);
    }
}

/// Dark and light color palettes bundled together.
#[derive(Debug, Clone)]
pub struct ThemeColors {
    pub dark:  ColorPalette,
    pub light: ColorPalette,
}

impl ThemeColors {
    /// Build from the Leonux default palette at a given primary hue (OKLCH).
    pub fn from_hue(hue_degrees: f32) -> Self {
        let gen = AccentGenerator::new(hue_degrees);
        Self {
            dark:  gen.build_dark(),
            light: gen.build_light(),
        }
    }
}
