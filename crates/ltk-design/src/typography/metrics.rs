//! Text metrics: advance widths, line heights, cap heights.

/// Measurements for a shaped text run.
#[derive(Debug, Clone, Copy, Default)]
pub struct TextMetrics {
    pub advance_width:  f32,   // total horizontal advance
    pub ascent:         f32,   // above baseline
    pub descent:        f32,   // below baseline (positive = down)
    pub line_height:    f32,   // ascent + descent + line gap
    pub cap_height:     f32,   // top of capital letters
    pub x_height:       f32,   // top of lowercase 'x'
    pub glyph_count:    u32,
}

impl TextMetrics {
    pub fn height(self) -> f32 { self.ascent + self.descent }
}
