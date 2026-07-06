//! Wayland wl_surface lifecycle management.

use ltk_core::id::SurfaceId;
use ltk_core::geometry::Size;

pub struct SurfaceEntry { pub id: SurfaceId, pub size: Size, pub scale: f32 }

pub struct SurfaceManager { surfaces: std::collections::HashMap<SurfaceId, SurfaceEntry> }

impl SurfaceManager {
    pub fn new() -> Self { Self { surfaces: std::collections::HashMap::new() } }
    pub fn create(&mut self, size: Size, scale: f32) -> SurfaceId {
        let id = SurfaceId::new();
        self.surfaces.insert(id, SurfaceEntry { id, size, scale });
        id
    }
    pub fn destroy(&mut self, id: SurfaceId) { self.surfaces.remove(&id); }
    pub fn get(&self, id: SurfaceId) -> Option<&SurfaceEntry> { self.surfaces.get(&id) }
    pub fn resize(&mut self, id: SurfaceId, size: Size) {
        if let Some(s) = self.surfaces.get_mut(&id) { s.size = size; }
    }
}

impl Default for SurfaceManager { fn default() -> Self { Self::new() } }
