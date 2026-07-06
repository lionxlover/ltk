//! Decoded image cache (RGBA8 pixels).

use ltk_core::{id::ResourceId, geometry::Size};
use std::collections::HashMap;

pub struct CachedImage { pub width: u32, pub height: u32, pub pixels: Vec<u8> }

impl CachedImage {
    pub fn size(&self) -> Size { Size::new(self.width as f32, self.height as f32) }
    pub fn byte_size(&self) -> usize { self.pixels.len() }
}

pub struct ImageCache { entries: HashMap<ResourceId, CachedImage>, total_bytes: usize, max_bytes: usize }

impl ImageCache {
    pub fn new(max_bytes: usize) -> Self { Self { entries: HashMap::new(), total_bytes: 0, max_bytes } }
    pub fn insert(&mut self, id: ResourceId, img: CachedImage) {
        self.total_bytes += img.byte_size();
        self.entries.insert(id, img);
    }
    pub fn get(&self, id: ResourceId) -> Option<&CachedImage> { self.entries.get(&id) }
    pub fn remove(&mut self, id: ResourceId) {
        if let Some(e) = self.entries.remove(&id) { self.total_bytes -= e.byte_size(); }
    }
    pub fn total_bytes(&self) -> usize { self.total_bytes }
}
