//! Dynamic color: wallpaper/image-extracted accent propagation.

use ltk_core::Color;

/// Extracts a dominant/vibrant accent color from image pixel data.
pub struct DynamicColor;

impl DynamicColor {
    /// Extract primary accent hue from an RGBA8 image buffer.
    /// Returns an OKLCH hue angle suitable for `AccentGenerator::new`.
    pub fn extract_hue(pixels: &[u8], width: u32, height: u32) -> Option<f32> {
        if pixels.is_empty() || width == 0 || height == 0 { return None; }
        // Simplified: average all pixels and derive hue from dominant channel.
        // A real implementation would use k-means clustering in OKLCH.
        let (mut r, mut g, mut b, mut count) = (0u64, 0u64, 0u64, 0u64);
        let len = pixels.len();
        let mut i = 0;
        while i + 3 < len {
            r += pixels[i] as u64;
            g += pixels[i+1] as u64;
            b += pixels[i+2] as u64;
            count += 1;
            i += 4;
        }
        if count == 0 { return None; }
        let (rf, gf, bf) = (r as f32 / count as f32 / 255.0,
                            g as f32 / count as f32 / 255.0,
                            b as f32 / count as f32 / 255.0);
        // Derive hue from RGB (simplified, not OKLCH)
        let max = rf.max(gf).max(bf);
        let min = rf.min(gf).min(bf);
        let delta = max - min;
        if delta < 0.05 { return None; } // achromatic
        let hue = if max == rf {
            60.0 * (((gf - bf) / delta) % 6.0)
        } else if max == gf {
            60.0 * ((bf - rf) / delta + 2.0)
        } else {
            60.0 * ((rf - gf) / delta + 4.0)
        };
        Some((hue + 360.0) % 360.0)
    }
}
