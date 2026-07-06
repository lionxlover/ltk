//! Unicode BiDi algorithm support and RTL layout mirroring.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextDirection { LTR, RTL, Auto }

/// A text run with resolved BiDi direction.
#[derive(Debug, Clone)]
pub struct BiDiText {
    pub text:      String,
    pub direction: TextDirection,
    pub level:     u8,   // BiDi embedding level
}

impl BiDiText {
    /// Analyse a string and return segments with their resolved directions.
    pub fn analyse(text: &str) -> Vec<Self> {
        use unicode_bidi::{BidiInfo, Level};
        let bidi  = BidiInfo::new(text, None);
        let para  = &bidi.paragraphs[0];
        let level = para.level;
        let dir   = if level.is_rtl() { TextDirection::RTL } else { TextDirection::LTR };
        vec![BiDiText { text: text.to_string(), direction: dir, level: level.number() }]
    }

    pub fn is_rtl(&self) -> bool { self.direction == TextDirection::RTL }
    pub fn is_ltr(&self) -> bool { self.direction == TextDirection::LTR }
}

/// Should a layout be mirrored for RTL locales?
pub fn should_mirror_layout(locale: &str) -> bool {
    matches!(locale.split('-').next().unwrap_or(""), "ar"|"he"|"fa"|"ur"|"ps")
}
