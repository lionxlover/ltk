//! Layer compositing order management.

use ltk_core::id::LayerId;
use smallvec::SmallVec;

pub struct LayerEntry { pub id: LayerId, pub z_index: i32, pub opacity: f32 }

pub struct LayerManager { layers: Vec<LayerEntry> }

impl LayerManager {
    pub fn new() -> Self { Self { layers: Vec::new() } }

    pub fn add(&mut self, id: LayerId, z_index: i32, opacity: f32) {
        self.layers.push(LayerEntry { id, z_index, opacity });
        self.layers.sort_by_key(|l| l.z_index);
    }

    pub fn remove(&mut self, id: LayerId) { self.layers.retain(|l| l.id != id); }
    pub fn iter_sorted(&self) -> std::slice::Iter<LayerEntry> { self.layers.iter() }
}

impl Default for LayerManager { fn default() -> Self { Self::new() } }
