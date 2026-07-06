//! DPI-aware text size scaling.

/// Adjusts a design-time font size for the actual display scale factor.
pub struct TextScaling {
    pub scale:           f32,   // DPI scale factor (1.0, 1.5, 2.0…)
    pub user_preference: f32,   // Multiplier from accessibility settings (0.8–2.0)
}

impl TextScaling {
    pub fn new(scale: f32, user_pref: f32) -> Self { Self { scale, user_preference: user_pref } }

    /// Convert a logical font size to physical pixels.
    pub fn to_physical(&self, logical_px: f32) -> f32 {
        logical_px * self.scale * self.user_preference
    }

    /// Convert physical pixels back to logical.
    pub fn to_logical(&self, physical_px: f32) -> f32 {
        physical_px / (self.scale * self.user_preference)
    }
}
