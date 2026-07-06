//! Algorithmic OKLCH-based color palette generation from a single hue.

use ltk_core::Color;
use super::{ColorToken, ColorPalette};
use std::collections::HashMap;

/// Generates a complete dark+light palette from a single hue angle (0–360).
pub struct AccentGenerator { hue: f32 }

impl AccentGenerator {
    pub fn new(hue_degrees: f32) -> Self { Self { hue: hue_degrees } }

    fn primary(&self, l: f32, c: f32) -> Color {
        Color::from_oklch(l, c, self.hue, 1.0)
    }

    /// Build the dark-mode palette.
    pub fn build_dark(&self) -> ColorPalette {
        let mut m = HashMap::new();
        use ColorToken::*;

        // Surfaces
        m.insert(BgBase,           Color::from_hex("#0B0D0F").unwrap());
        m.insert(BgSurface,        Color::from_hex("#15171B").unwrap());
        m.insert(BgSurfaceRaised,  Color::from_hex("#1C1F23").unwrap());
        m.insert(BgOverlay,        Color::from_hex("#2A2F38").unwrap());

        // Primary accent (generated from hue)
        let primary = self.primary(0.64, 0.18);
        m.insert(Primary,          primary);
        m.insert(PrimaryDim,       self.primary(0.52, 0.16));
        m.insert(PrimaryGlow,      primary.with_alpha(0.35));
        m.insert(PrimaryContainer, self.primary(0.22, 0.08));
        m.insert(OnPrimary,        Color::from_hex("#FFFFFF").unwrap());

        // Fixed accents
        m.insert(AccentPurple, Color::from_oklch(0.72, 0.18, 280.0, 1.0));
        m.insert(AccentGreen,  Color::from_oklch(0.75, 0.20, 148.0, 1.0));
        m.insert(AccentAmber,  Color::from_oklch(0.78, 0.18,  85.0, 1.0));
        m.insert(AccentRed,    Color::from_oklch(0.68, 0.20,  25.0, 1.0));

        // Text
        m.insert(TextPrimary,   Color::from_hex("#F4F5F7").unwrap());
        m.insert(TextSecondary, Color::from_hex("#9CA3AF").unwrap());
        m.insert(TextTertiary,  Color::from_hex("#5C6270").unwrap());
        m.insert(TextDisabled,  Color::from_hex("#3A3F4A").unwrap());
        m.insert(TextOnPrimary, Color::WHITE);

        // Borders
        m.insert(BorderSubtle,  Color::WHITE.with_alpha(0.06));
        m.insert(BorderBase,    Color::WHITE.with_alpha(0.10));
        m.insert(BorderStrong,  Color::WHITE.with_alpha(0.18));
        m.insert(BorderFocus,   primary.with_alpha(0.80));
        m.insert(BorderError,   Color::from_oklch(0.68, 0.20, 25.0, 0.60));

        // State colours
        let success = Color::from_oklch(0.75, 0.20, 148.0, 1.0);
        let warning = Color::from_oklch(0.78, 0.18, 85.0, 1.0);
        let error   = Color::from_oklch(0.68, 0.20, 25.0, 1.0);
        let info    = primary;

        m.insert(StateSuccess,   success);
        m.insert(StateSuccessBg, success.with_alpha(0.12));
        m.insert(StateWarning,   warning);
        m.insert(StateWarningBg, warning.with_alpha(0.12));
        m.insert(StateError,     error);
        m.insert(StateErrorBg,   error.with_alpha(0.12));
        m.insert(StateInfo,      info);
        m.insert(StateInfoBg,    info.with_alpha(0.12));

        // Shadows
        m.insert(ShadowSm, Color::BLACK.with_alpha(0.20));
        m.insert(ShadowMd, Color::BLACK.with_alpha(0.35));
        m.insert(ShadowLg, Color::BLACK.with_alpha(0.50));
        m.insert(ShadowXl, Color::BLACK.with_alpha(0.65));
        m.insert(Scrim,    Color::BLACK.with_alpha(0.70));

        ColorPalette(m)
    }

    /// Build the light-mode palette.
    pub fn build_light(&self) -> ColorPalette {
        let mut m = HashMap::new();
        use ColorToken::*;

        m.insert(BgBase,           Color::from_hex("#F3F2EF").unwrap());
        m.insert(BgSurface,        Color::from_hex("#FFFFFF").unwrap());
        m.insert(BgSurfaceRaised,  Color::from_hex("#F0F2F5").unwrap());
        m.insert(BgOverlay,        Color::from_hex("#DDE0E5").unwrap());

        let primary = self.primary(0.48, 0.20);
        m.insert(Primary,          primary);
        m.insert(PrimaryDim,       self.primary(0.40, 0.18));
        m.insert(PrimaryGlow,      primary.with_alpha(0.25));
        m.insert(PrimaryContainer, self.primary(0.90, 0.08));
        m.insert(OnPrimary,        Color::WHITE);

        m.insert(AccentPurple, Color::from_oklch(0.50, 0.20, 280.0, 1.0));
        m.insert(AccentGreen,  Color::from_oklch(0.45, 0.22, 148.0, 1.0));
        m.insert(AccentAmber,  Color::from_oklch(0.52, 0.20,  85.0, 1.0));
        m.insert(AccentRed,    Color::from_oklch(0.48, 0.22,  25.0, 1.0));

        m.insert(TextPrimary,   Color::from_hex("#15171B").unwrap());
        m.insert(TextSecondary, Color::from_hex("#5B6168").unwrap());
        m.insert(TextTertiary,  Color::from_hex("#9499A1").unwrap());
        m.insert(TextDisabled,  Color::from_hex("#C4C9D4").unwrap());
        m.insert(TextOnPrimary, Color::WHITE);

        m.insert(BorderSubtle,  Color::BLACK.with_alpha(0.06));
        m.insert(BorderBase,    Color::BLACK.with_alpha(0.10));
        m.insert(BorderStrong,  Color::BLACK.with_alpha(0.18));
        m.insert(BorderFocus,   primary.with_alpha(0.80));
        m.insert(BorderError,   Color::from_oklch(0.48, 0.22, 25.0, 0.60));

        let success = Color::from_oklch(0.45, 0.22, 148.0, 1.0);
        let warning = Color::from_oklch(0.52, 0.20, 85.0, 1.0);
        let error   = Color::from_oklch(0.48, 0.22, 25.0, 1.0);
        let info    = primary;

        m.insert(StateSuccess, success); m.insert(StateSuccessBg, success.with_alpha(0.10));
        m.insert(StateWarning, warning); m.insert(StateWarningBg, warning.with_alpha(0.10));
        m.insert(StateError,   error);   m.insert(StateErrorBg,   error.with_alpha(0.10));
        m.insert(StateInfo,    info);    m.insert(StateInfoBg,    info.with_alpha(0.10));

        m.insert(ShadowSm, Color::BLACK.with_alpha(0.06));
        m.insert(ShadowMd, Color::BLACK.with_alpha(0.12));
        m.insert(ShadowLg, Color::BLACK.with_alpha(0.20));
        m.insert(ShadowXl, Color::BLACK.with_alpha(0.30));
        m.insert(Scrim,    Color::BLACK.with_alpha(0.55));

        ColorPalette(m)
    }
}
