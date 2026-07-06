//! Spacing token system on an 8px base grid.

use serde::{Deserialize, Serialize};

/// Named spacing tokens on the 8px grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SpaceToken {
    Px1, Px2, Px4,
    S0_5, S1, S1_5, S2, S2_5, S3, S3_5,
    S4, S5, S6, S7, S8, S10, S12, S14, S16, S20, S24,
}

impl SpaceToken {
    /// Resolve to logical pixels using the 8px base unit.
    pub fn logical_px(self) -> f32 {
        use SpaceToken::*;
        match self {
            Px1  => 1.0,  Px2 => 2.0,  Px4 => 4.0,
            S0_5 => 4.0,
            S1   => 8.0,  S1_5 => 12.0, S2  => 16.0, S2_5 => 20.0,
            S3   => 24.0, S3_5 => 28.0, S4  => 32.0,
            S5   => 40.0, S6   => 48.0, S7  => 56.0,  S8  => 64.0,
            S10  => 80.0, S12  => 96.0, S14 => 112.0, S16 => 128.0,
            S20  => 160.0,S24  => 192.0,
        }
    }
}

/// The full spacing scale.
pub struct SpaceScale { pub base: f32 }

impl SpaceScale {
    pub fn new(base_px: f32) -> Self { Self { base: base_px } }
    pub fn resolve(&self, token: SpaceToken) -> f32 {
        token.logical_px() * (self.base / 8.0)
    }
}

/// Layout-level tokens: column count, gutter, max-widths, breakpoints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutTokens {
    pub grid_columns:   u32,         // 12
    pub grid_gutter:    f32,         // 24.0
    pub max_width_sm:   f32,         // 640
    pub max_width_md:   f32,         // 768
    pub max_width_lg:   f32,         // 1024
    pub max_width_xl:   f32,         // 1280
    pub max_width_2xl:  f32,         // 1536
}

impl Default for LayoutTokens {
    fn default() -> Self {
        Self {
            grid_columns: 12, grid_gutter: 24.0,
            max_width_sm:  640.0, max_width_md:  768.0,
            max_width_lg: 1024.0, max_width_xl: 1280.0,
            max_width_2xl: 1536.0,
        }
    }
}
