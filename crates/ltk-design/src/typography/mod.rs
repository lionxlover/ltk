//! Typography token system: font registry, specs, text styles.

pub mod registry;
pub mod fallback;
pub mod metrics;
pub mod scaling;

pub use registry::{FontRegistry, FontId, FontFaceInfo};
pub use fallback::FontFallbackChain;
pub use metrics::TextMetrics;
pub use scaling::TextScaling;

use serde::{Deserialize, Serialize};

// ── FontWeight ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u16)]
pub enum FontWeight {
    Thin       = 100,
    ExtraLight = 200,
    Light      = 300,
    Regular    = 400,
    Medium     = 500,
    SemiBold   = 600,
    Bold       = 700,
    ExtraBold  = 800,
    Black      = 900,
}

impl FontWeight {
    pub fn value(self) -> u16 { self as u16 }
}

impl Default for FontWeight {
    fn default() -> Self { Self::Regular }
}

// ── FontStyle ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum FontStyle { #[default] Normal, Italic, Oblique }

// ── TextDecoration ────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum TextDecoration { #[default] None, Underline, StrikeThrough, Overline }

// ── FontSpec ─────────────────────────────────────────────────────────

/// A fully-resolved font specification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FontSpec {
    pub families:  Vec<String>,     // ordered priority
    pub size:      f32,             // logical px
    pub weight:    FontWeight,
    pub style:     FontStyle,
    pub stretch:   f32,             // 1.0 = 100%
    pub line_height: Option<f32>,   // None = auto (1.2×)
    pub letter_spacing: f32,        // em units
}

impl FontSpec {
    pub fn body(size: f32) -> Self {
        Self {
            families: vec!["Inter".into()],
            size,
            weight: FontWeight::Regular,
            style: FontStyle::Normal,
            stretch: 1.0,
            line_height: None,
            letter_spacing: 0.0,
        }
    }

    pub fn display(size: f32) -> Self {
        Self {
            families: vec!["Space Grotesk".into()],
            size,
            weight: FontWeight::Bold,
            style: FontStyle::Normal,
            stretch: 1.0,
            line_height: Some(1.1),
            letter_spacing: -0.03,
        }
    }

    pub fn mono(size: f32) -> Self {
        Self {
            families: vec!["JetBrains Mono".into()],
            size,
            weight: FontWeight::Regular,
            style: FontStyle::Normal,
            stretch: 1.0,
            line_height: Some(1.6),
            letter_spacing: 0.0,
        }
    }
}

// ── TextStyle ────────────────────────────────────────────────────────

/// A named text style token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextStyle {
    DisplayLg, DisplayMd, DisplaySm,
    HeadingXl, HeadingLg, HeadingMd, HeadingSm,
    Lead,
    BodyLg, BodyMd, BodySm,
    LabelLg, LabelMd, LabelSm,
    CodeLg, CodeMd, CodeSm,
    Caption, Overline,
}

// ── TypeScale ────────────────────────────────────────────────────────

/// Resolves `TextStyle` tokens to `FontSpec` at a given base size.
pub struct TypeScale { pub base: f32 }

impl TypeScale {
    pub fn new(base_px: f32) -> Self { Self { base: base_px } }

    pub fn resolve(&self, style: TextStyle) -> FontSpec {
        use TextStyle::*;
        let b = self.base;
        match style {
            DisplayLg  => FontSpec::display(b * 3.25),
            DisplayMd  => FontSpec::display(b * 2.50),
            DisplaySm  => FontSpec::display(b * 2.00),
            HeadingXl  => FontSpec::display(b * 1.75),
            HeadingLg  => FontSpec::display(b * 1.50),
            HeadingMd  => FontSpec::display(b * 1.25),
            HeadingSm  => FontSpec::display(b * 1.10),
            Lead       => { let mut s = FontSpec::body(b * 1.125); s.line_height = Some(1.65); s },
            BodyLg     => FontSpec::body(b),
            BodyMd     => FontSpec::body(b * 0.875),
            BodySm     => FontSpec::body(b * 0.8125),
            LabelLg    => { let mut s = FontSpec::body(b * 0.875);  s.weight = FontWeight::Medium; s },
            LabelMd    => { let mut s = FontSpec::body(b * 0.8125); s.weight = FontWeight::Medium; s },
            LabelSm    => { let mut s = FontSpec::body(b * 0.75);   s.weight = FontWeight::SemiBold; s.letter_spacing = 0.06; s },
            CodeLg     => FontSpec::mono(b),
            CodeMd     => FontSpec::mono(b * 0.875),
            CodeSm     => FontSpec::mono(b * 0.8125),
            Caption    => { let mut s = FontSpec::body(b * 0.75); s.line_height = Some(1.5); s },
            Overline   => { let mut s = FontSpec::body(b * 0.6875); s.weight = FontWeight::SemiBold; s.letter_spacing = 0.12; s },
        }
    }
}
