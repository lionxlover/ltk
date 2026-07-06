//! Color representation in linear sRGB with OKLCH conversion utilities.

use std::fmt;

/// Floating-point RGBA color in **linear** sRGB (not gamma-encoded).
/// All internal math operates on linear values; convert at paint time.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Color { r: f32, g: f32, b: f32, a: f32 }

impl Color {
    // ── Named constants ──────────────────────────────────────────────
    pub const BLACK:       Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const WHITE:       Self = Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const TRANSPARENT: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };

    // ── Constructors ─────────────────────────────────────────────────

    /// Create from linear sRGB components (0.0–1.0).
    #[inline] pub const fn linear(r: f32, g: f32, b: f32, a: f32) -> Self { Self { r, g, b, a } }

    /// Create from gamma-encoded sRGB bytes (0–255).
    #[inline] pub fn from_srgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: srgb_to_linear(r as f32 / 255.0),
            g: srgb_to_linear(g as f32 / 255.0),
            b: srgb_to_linear(b as f32 / 255.0),
            a: 1.0,
        }
    }

    /// Create from sRGB bytes with alpha (0–255).
    #[inline] pub fn from_srgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        let mut c = Self::from_srgb(r, g, b);
        c.a = a as f32 / 255.0;
        c
    }

    /// Create from a `#RRGGBB` or `#RRGGBBAA` hex string.
    pub fn from_hex(hex: &str) -> Result<Self, ColorParseError> {
        let s = hex.trim_start_matches('#');
        let parse = |i: usize| u8::from_str_radix(&s[i..i+2], 16).map_err(|_| ColorParseError::InvalidHex);
        match s.len() {
            6 => Ok(Self::from_srgb(parse(0)?, parse(2)?, parse(4)?)),
            8 => Ok(Self::from_srgba(parse(0)?, parse(2)?, parse(4)?, parse(6)?)),
            _ => Err(ColorParseError::InvalidLength),
        }
    }

    /// Create from OKLCH (perceptually uniform) values.
    /// L = lightness 0.0–1.0, C = chroma 0.0–0.4+, H = hue 0–360.
    pub fn from_oklch(l: f32, c: f32, h: f32, a: f32) -> Self {
        let h_rad = h.to_radians();
        let ok_a = c * h_rad.cos();
        let ok_b = c * h_rad.sin();
        // OKLab → linear sRGB (Björn Ottosson's matrix)
        let l_ = l + 0.3963377774 * ok_a + 0.2158037573 * ok_b;
        let m_ = l - 0.1055613458 * ok_a - 0.0638541728 * ok_b;
        let s_ = l - 0.0894841775 * ok_a - 1.2914855480 * ok_b;
        let (l3, m3, s3) = (l_.powi(3), m_.powi(3), s_.powi(3));
        Self {
            r: ( 4.0767416621 * l3 - 3.3077115913 * m3 + 0.2309699292 * s3).clamp(0.0, 1.0),
            g: (-1.2684380046 * l3 + 2.6097574011 * m3 - 0.3413193965 * s3).clamp(0.0, 1.0),
            b: (-0.0041960863 * l3 - 0.7034186147 * m3 + 1.7076147010 * s3).clamp(0.0, 1.0),
            a,
        }
    }

    // ── Accessors ────────────────────────────────────────────────────
    #[inline] pub fn r(self) -> f32 { self.r }
    #[inline] pub fn g(self) -> f32 { self.g }
    #[inline] pub fn b(self) -> f32 { self.b }
    #[inline] pub fn alpha(self) -> f32 { self.a }

    /// Return as gamma-encoded sRGB bytes.
    #[inline] pub fn to_srgb_bytes(self) -> [u8; 4] {
        [
            (linear_to_srgb(self.r) * 255.0).round() as u8,
            (linear_to_srgb(self.g) * 255.0).round() as u8,
            (linear_to_srgb(self.b) * 255.0).round() as u8,
            (self.a * 255.0).round() as u8,
        ]
    }

    // ── Manipulations ────────────────────────────────────────────────

    #[inline] pub fn with_alpha(self, a: f32) -> Self { Self { a, ..self } }
    #[inline] pub fn multiply_alpha(self, factor: f32) -> Self { Self { a: (self.a * factor).clamp(0.0, 1.0), ..self } }

    /// Linear interpolation between two colors.
    #[inline] pub fn lerp(self, other: Self, t: f32) -> Self {
        Self {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }

    /// Porter-Duff "source over" composite.
    #[inline] pub fn over(self, dst: Self) -> Self {
        let out_a = self.a + dst.a * (1.0 - self.a);
        if out_a < 1e-6 { return Self::TRANSPARENT; }
        Self {
            r: (self.r * self.a + dst.r * dst.a * (1.0 - self.a)) / out_a,
            g: (self.g * self.a + dst.g * dst.a * (1.0 - self.a)) / out_a,
            b: (self.b * self.a + dst.b * dst.a * (1.0 - self.a)) / out_a,
            a: out_a,
        }
    }

    // ── Accessibility ────────────────────────────────────────────────

    /// Relative luminance per WCAG 2.1.
    pub fn relative_luminance(self) -> f32 {
        0.2126 * self.r + 0.7152 * self.g + 0.0722 * self.b
    }

    /// WCAG contrast ratio (both colors assumed opaque).
    pub fn contrast_ratio(self, other: Self) -> f32 {
        let l1 = self.relative_luminance().max(other.relative_luminance());
        let l2 = self.relative_luminance().min(other.relative_luminance());
        (l1 + 0.05) / (l2 + 0.05)
    }

    /// Returns true if contrast meets WCAG AA (normal text: ≥4.5).
    pub fn is_wcag_aa(self, bg: Self) -> bool { self.contrast_ratio(bg) >= 4.5 }

    /// Returns true if contrast meets WCAG AAA (normal text: ≥7.0).
    pub fn is_wcag_aaa(self, bg: Self) -> bool { self.contrast_ratio(bg) >= 7.0 }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let b = self.to_srgb_bytes();
        write!(f, "#{:02X}{:02X}{:02X}{:02X}", b[0], b[1], b[2], b[3])
    }
}

// ── Gamma conversion ─────────────────────────────────────────────────

#[inline] fn srgb_to_linear(v: f32) -> f32 {
    if v <= 0.04045 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) }
}

#[inline] fn linear_to_srgb(v: f32) -> f32 {
    let v = v.clamp(0.0, 1.0);
    if v <= 0.0031308 { v * 12.92 } else { 1.055 * v.powf(1.0 / 2.4) - 0.055 }
}

// ── Errors ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum ColorParseError {
    #[error("Invalid hex color length — expected 6 or 8 hex digits")]
    InvalidLength,
    #[error("Invalid hex digit in color string")]
    InvalidHex,
}
