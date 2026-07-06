//! SDF glyph cache for GPU text rendering.

use std::collections::HashMap;

/// A cached glyph entry in the atlas.
#[derive(Debug, Clone, Copy)]
pub struct GlyphEntry {
    pub atlas_x:   u32,
    pub atlas_y:   u32,
    pub width:     u32,
    pub height:    u32,
    pub bearing_x: i32,
    pub bearing_y: i32,
    pub advance:   f32,
}

pub struct GlyphCache {
    atlas_size: u32,
    entries:    HashMap<(u32, u32), GlyphEntry>, // (font_id, glyph_id)
    cursor_x:   u32,
    cursor_y:   u32,
    row_height: u32,
}

impl GlyphCache {
    pub fn new(atlas_size: u32) -> Self {
        Self { atlas_size, entries: HashMap::new(), cursor_x: 0, cursor_y: 0, row_height: 0 }
    }

    pub fn get(&self, font_id: u32, glyph_id: u32) -> Option<&GlyphEntry> {
        self.entries.get(&(font_id, glyph_id))
    }

    pub fn insert(&mut self, font_id: u32, glyph_id: u32, entry: GlyphEntry) {
        self.entries.insert((font_id, glyph_id), entry);
    }

    pub fn glyph_count(&self) -> usize { self.entries.len() }
}

impl Default for GlyphCache { fn default() -> Self { Self::new(4096) } }
