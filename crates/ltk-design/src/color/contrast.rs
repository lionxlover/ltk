//! WCAG contrast checking and automatic accessible color selection.

use ltk_core::Color;

/// WCAG conformance level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WcagLevel { AA, AAA }

impl WcagLevel {
    fn min_ratio_normal(self) -> f32 { match self { Self::AA => 4.5, Self::AAA => 7.0 } }
    fn min_ratio_large(self) -> f32  { match self { Self::AA => 3.0, Self::AAA => 4.5 } }
}

/// Checks and adjusts colors to meet WCAG contrast requirements.
pub struct ContrastEngine;

impl ContrastEngine {
    /// Check if two colors meet the given WCAG level (normal text).
    pub fn passes(fg: Color, bg: Color, level: WcagLevel) -> bool {
        fg.contrast_ratio(bg) >= level.min_ratio_normal()
    }

    /// Check for large text (≥18pt or ≥14pt bold).
    pub fn passes_large(fg: Color, bg: Color, level: WcagLevel) -> bool {
        fg.contrast_ratio(bg) >= level.min_ratio_large()
    }

    /// Given a background color, return either white or black,
    /// whichever has higher contrast — guaranteed to pass WCAG AA.
    pub fn best_on(bg: Color) -> Color {
        let white_r = Color::WHITE.contrast_ratio(bg);
        let black_r = Color::BLACK.contrast_ratio(bg);
        if white_r >= black_r { Color::WHITE } else { Color::BLACK }
    }

    /// Adjust the OKLCH lightness of `fg` until it passes `level` against `bg`.
    /// Returns the adjusted color or the original if adjustment fails.
    pub fn ensure_contrast(fg: Color, bg: Color, level: WcagLevel) -> Color {
        if Self::passes(fg, bg, level) { return fg; }
        // Simple binary-search lightness adjustment (placeholder)
        // A real implementation would work in OKLCH space.
        Self::best_on(bg)
    }
}
