//! Shaped text / glyph run cache.

use ltk_core::string::SharedString;
use std::collections::HashMap;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct TextCacheKey { pub text: SharedString, pub font_id: u32, pub size_px: u32 }

/// A shaped text run (glyph indices + positions).
#[derive(Debug, Clone)]
pub struct ShapedRun {
    pub advance_width: f32,
    pub glyphs: Vec<(u32, f32, f32)>,   // (glyph_id, x, y)
}

pub struct FontCache { entries: HashMap<TextCacheKey, ShapedRun>, count: usize }

impl FontCache {
    pub fn new() -> Self { Self { entries: HashMap::new(), count: 0 } }
    pub fn get(&self, key: &TextCacheKey) -> Option<&ShapedRun> { self.entries.get(key) }
    pub fn insert(&mut self, key: TextCacheKey, run: ShapedRun) {
        self.entries.insert(key, run); self.count += 1;
    }
    pub fn clear(&mut self) { self.entries.clear(); }
    pub fn count(&self) -> usize { self.count }
}

impl Default for FontCache { fn default() -> Self { Self::new() } }
