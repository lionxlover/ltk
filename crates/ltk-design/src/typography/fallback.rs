//! Script-aware font fallback chains.

/// A font fallback chain for a given script/locale.
#[derive(Debug, Clone)]
pub struct FontFallbackChain {
    pub primary:  String,
    pub fallbacks: Vec<String>,
    pub emoji:    String,
    pub symbols:  Vec<String>,
}

impl FontFallbackChain {
    /// Default chain for Latin/English text.
    pub fn latin() -> Self {
        Self {
            primary: "Inter".into(),
            fallbacks: vec!["Noto Sans".into(), "Liberation Sans".into(), "DejaVu Sans".into()],
            emoji: "Noto Color Emoji".into(),
            symbols: vec!["Noto Sans Symbols".into(), "Noto Sans Symbols 2".into()],
        }
    }

    /// Chain for CJK (Chinese/Japanese/Korean).
    pub fn cjk() -> Self {
        Self {
            primary: "Noto Sans CJK".into(),
            fallbacks: vec!["WenQuanYi Micro Hei".into()],
            emoji: "Noto Color Emoji".into(),
            symbols: vec![],
        }
    }

    /// Chain for Arabic/Persian/Urdu (RTL).
    pub fn arabic() -> Self {
        Self {
            primary: "Noto Sans Arabic".into(),
            fallbacks: vec!["DejaVu Sans".into()],
            emoji: "Noto Color Emoji".into(),
            symbols: vec![],
        }
    }

    /// Build the full ordered family list for CSS/fontdb.
    pub fn all_families(&self) -> Vec<&str> {
        let mut v: Vec<&str> = vec![&self.primary];
        v.extend(self.fallbacks.iter().map(|s| s.as_str()));
        v.push(&self.emoji);
        v.extend(self.symbols.iter().map(|s| s.as_str()));
        v
    }
}
