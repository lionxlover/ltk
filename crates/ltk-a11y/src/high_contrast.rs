//! High-contrast mode detection and theme overrides.

use ltk_core::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighContrastMode { None, Black, White, Custom }

pub struct HighContrastEngine { pub mode: HighContrastMode }

impl HighContrastEngine {
    pub fn detect() -> Self {
        // Real impl: read gsettings org.gnome.desktop.a11y.interface high-contrast
        Self { mode: HighContrastMode::None }
    }

    pub fn apply_override(&self, color: Color) -> Color {
        match self.mode {
            HighContrastMode::None   => color,
            HighContrastMode::Black  => if color.relative_luminance() > 0.5 { Color::BLACK } else { Color::WHITE },
            HighContrastMode::White  => if color.relative_luminance() < 0.5 { Color::WHITE } else { Color::BLACK },
            HighContrastMode::Custom => color,
        }
    }

    pub fn is_active(&self) -> bool { self.mode != HighContrastMode::None }
}
