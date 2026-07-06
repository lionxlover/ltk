//! Locale detection and priority list management.

use serde::{Deserialize, Serialize};

/// A BCP 47 locale identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Locale(pub String);

impl Locale {
    pub fn new(id: impl Into<String>) -> Self { Self(id.into()) }
    pub fn language(&self) -> &str { self.0.split('-').next().unwrap_or("") }
    pub fn region(&self) -> Option<&str> { self.0.split('-').nth(1) }
    pub fn as_str(&self) -> &str { &self.0 }
    pub fn is_rtl(&self) -> bool {
        matches!(self.language(), "ar"|"he"|"fa"|"ur"|"ps"|"ku"|"sd")
    }
}

impl std::fmt::Display for Locale { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.0.fmt(f) } }

/// Manages the user's locale preference list with fallback chain.
pub struct LangManager {
    pub preferred: Vec<Locale>,
    pub system:    Locale,
}

impl LangManager {
    pub fn new(system: Locale) -> Self {
        let preferred = vec![system.clone()];
        Self { preferred, system }
    }

    /// Detect locale from environment variables.
    pub fn detect() -> Self {
        let raw = std::env::var("LANG")
            .or_else(|_| std::env::var("LANGUAGE"))
            .unwrap_or_else(|_| "en_US.UTF-8".into());
        let id  = raw.split('.').next().unwrap_or("en_US").replace('_', "-");
        Self::new(Locale::new(id))
    }

    /// Get the active locale (first in preferred list).
    pub fn active(&self) -> &Locale { self.preferred.first().unwrap_or(&self.system) }

    /// Resolve the best available locale given a list of available locales.
    pub fn best_match<'a>(&self, available: &'a [Locale]) -> Option<&'a Locale> {
        for pref in &self.preferred {
            if let Some(exact) = available.iter().find(|a| a == &pref) { return Some(exact); }
        }
        // Try language-only match
        for pref in &self.preferred {
            let lang = pref.language();
            if let Some(m) = available.iter().find(|a| a.language() == lang) { return Some(m); }
        }
        None
    }
}
