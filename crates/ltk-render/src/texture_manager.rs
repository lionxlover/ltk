//! GPU texture atlas with LRU eviction.

use ltk_core::id::TextureId;
use ltk_core::geometry::Size;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TextureEntry {
    pub id:      TextureId,
    pub size:    Size,
    pub mips:    bool,
    pub last_used: u64, // frame number
}

pub struct TextureManager {
    textures:  HashMap<TextureId, TextureEntry>,
    budget_mb: f32,
    frame_nr:  u64,
}

impl TextureManager {
    pub fn new(budget_mb: f32) -> Self {
        Self { textures: HashMap::new(), budget_mb, frame_nr: 0 }
    }

    pub fn allocate(&mut self, size: Size, mips: bool) -> TextureId {
        let id = TextureId::new();
        self.textures.insert(id, TextureEntry { id, size, mips, last_used: self.frame_nr });
        id
    }

    pub fn mark_used(&mut self, id: TextureId) {
        if let Some(e) = self.textures.get_mut(&id) { e.last_used = self.frame_nr; }
    }

    pub fn free(&mut self, id: TextureId) { self.textures.remove(&id); }

    /// Evict textures not used in the last `age_frames` frames.
    pub fn evict(&mut self, age_frames: u64) {
        let cutoff = self.frame_nr.saturating_sub(age_frames);
        self.textures.retain(|_, e| e.last_used >= cutoff);
    }

    pub fn advance_frame(&mut self) { self.frame_nr += 1; }
    pub fn count(&self) -> usize { self.textures.len() }
}

impl Default for TextureManager { fn default() -> Self { Self::new(256.0) } }
