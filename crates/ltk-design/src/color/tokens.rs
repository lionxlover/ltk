//! Named color token aliases (semantic sugar over raw ColorToken).

use ltk_core::Color;
use super::{ColorToken, ColorPalette};

/// Convenience accessor wrapping a palette reference.
pub struct ColorTokens<'p>(&'p ColorPalette);

impl<'p> ColorTokens<'p> {
    pub fn new(palette: &'p ColorPalette) -> Self { Self(palette) }

    #[inline] pub fn primary(&self)          -> Color { self.0.get(ColorToken::Primary) }
    #[inline] pub fn on_primary(&self)       -> Color { self.0.get(ColorToken::OnPrimary) }
    #[inline] pub fn bg_base(&self)          -> Color { self.0.get(ColorToken::BgBase) }
    #[inline] pub fn bg_surface(&self)       -> Color { self.0.get(ColorToken::BgSurface) }
    #[inline] pub fn bg_raised(&self)        -> Color { self.0.get(ColorToken::BgSurfaceRaised) }
    #[inline] pub fn text_primary(&self)     -> Color { self.0.get(ColorToken::TextPrimary) }
    #[inline] pub fn text_secondary(&self)   -> Color { self.0.get(ColorToken::TextSecondary) }
    #[inline] pub fn text_tertiary(&self)    -> Color { self.0.get(ColorToken::TextTertiary) }
    #[inline] pub fn border_base(&self)      -> Color { self.0.get(ColorToken::BorderBase) }
    #[inline] pub fn border_focus(&self)     -> Color { self.0.get(ColorToken::BorderFocus) }
    #[inline] pub fn state_error(&self)      -> Color { self.0.get(ColorToken::StateError) }
    #[inline] pub fn state_success(&self)    -> Color { self.0.get(ColorToken::StateSuccess) }
    #[inline] pub fn accent_purple(&self)    -> Color { self.0.get(ColorToken::AccentPurple) }
    #[inline] pub fn accent_green(&self)     -> Color { self.0.get(ColorToken::AccentGreen) }
    #[inline] pub fn accent_amber(&self)     -> Color { self.0.get(ColorToken::AccentAmber) }
    #[inline] pub fn accent_red(&self)       -> Color { self.0.get(ColorToken::AccentRed) }
}
