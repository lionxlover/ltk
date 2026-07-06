//! Shadow, blur, and glass surface specifications.

use ltk_core::Color;
use serde::{Deserialize, Serialize};

/// Named elevation levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElevationLevel { None, Sm, Md, Lg, Xl, Dialog }

/// A drop-shadow specification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShadowSpec {
    pub offset_x: f32,
    pub offset_y: f32,
    pub blur:     f32,
    pub spread:   f32,
    pub color:    Color,
    pub inset:    bool,
}

impl ShadowSpec {
    pub fn at_elevation(level: ElevationLevel, shadow_color: Color) -> Self {
        use ElevationLevel::*;
        match level {
            None   => Self { offset_x: 0.0, offset_y: 0.0, blur:  0.0, spread: 0.0, color: Color::TRANSPARENT, inset: false },
            Sm     => Self { offset_x: 0.0, offset_y: 2.0, blur:  6.0, spread: 0.0, color: shadow_color.with_alpha(0.20), inset: false },
            Md     => Self { offset_x: 0.0, offset_y: 4.0, blur: 16.0, spread: 0.0, color: shadow_color.with_alpha(0.28), inset: false },
            Lg     => Self { offset_x: 0.0, offset_y: 8.0, blur: 32.0, spread: 0.0, color: shadow_color.with_alpha(0.38), inset: false },
            Xl     => Self { offset_x: 0.0, offset_y:16.0, blur: 56.0, spread: 0.0, color: shadow_color.with_alpha(0.50), inset: false },
            Dialog => Self { offset_x: 0.0, offset_y:24.0, blur: 72.0, spread: 0.0, color: shadow_color.with_alpha(0.60), inset: false },
        }
    }
}

/// Gaussian blur descriptor for backdrop-filter equivalents.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BlurSpec {
    pub radius: f32,   // logical px
    pub quality: BlurQuality,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlurQuality { Fast, Normal, High }

/// Full glass-surface specification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GlassSpec {
    pub blur:         BlurSpec,
    pub tint:         Color,       // usually bg-surface with 0.6–0.85 alpha
    pub border_color: Color,
    pub border_width: f32,
    pub specular:     f32,         // 0.0–1.0 top-edge highlight intensity
}

impl GlassSpec {
    /// Default dark glass surface.
    pub fn dark() -> Self {
        Self {
            blur:         BlurSpec { radius: 20.0, quality: BlurQuality::Normal },
            tint:         Color::from_hex("#15171B").unwrap().with_alpha(0.70),
            border_color: Color::WHITE.with_alpha(0.10),
            border_width: 1.0,
            specular:     0.06,
        }
    }

    /// Default light glass surface.
    pub fn light() -> Self {
        Self {
            blur:         BlurSpec { radius: 20.0, quality: BlurQuality::Normal },
            tint:         Color::WHITE.with_alpha(0.72),
            border_color: Color::BLACK.with_alpha(0.08),
            border_width: 1.0,
            specular:     0.14,
        }
    }
}
