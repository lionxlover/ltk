//! Translation engine: load and look up localised strings.

use ltk_core::string::SharedString;
use std::collections::HashMap;

/// A single translation entry.
#[derive(Debug, Clone)]
pub struct Translation {
    pub key:   SharedString,
    pub value: SharedString,
    pub note:  Option<String>,
}

/// Translation table for one locale.
pub struct TranslationTable { entries: HashMap<SharedString, String> }

impl TranslationTable {
    pub fn new() -> Self { Self { entries: HashMap::new() } }
    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.entries.insert(SharedString::new(key), value.into());
    }
    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.get(&SharedString::new(key)).map(|s| s.as_str())
    }
    pub fn load_json(&mut self, json: &str) -> Result<(), serde_json::Error> {
        let map: HashMap<String, String> = serde_json::from_str(json)?;
        for (k, v) in map { self.insert(k, v); }
        Ok(())
    }
}

/// Central localisation engine.
pub struct LocaleEngine {
    tables:   HashMap<String, TranslationTable>,
    locale:   String,
    fallback: String,
}

impl LocaleEngine {
    pub fn new(locale: impl Into<String>) -> Self {
        let locale = locale.into();
        Self { tables: HashMap::new(), fallback: "en-US".into(), locale }
    }

    pub fn add_table(&mut self, locale: impl Into<String>, table: TranslationTable) {
        self.tables.insert(locale.into(), table);
    }

    /// Translate a key. Falls back to English, then returns the key.
    pub fn t(&self, key: &str) -> &str {
        if let Some(v) = self.tables.get(&self.locale).and_then(|t| t.get(key)) { return v; }
        if let Some(v) = self.tables.get(&self.fallback).and_then(|t| t.get(key)) { return v; }
        key
    }

    /// Translate with named argument substitution `{name}`.
    pub fn t_args(&self, key: &str, args: &[(&str, &str)]) -> String {
        let mut s = self.t(key).to_string();
        for (k, v) in args { s = s.replace(&format!("{{{k}}}"), v); }
        s
    }

    pub fn locale(&self) -> &str { &self.locale }
    pub fn set_locale(&mut self, locale: impl Into<String>) { self.locale = locale.into(); }
}
