//! Multi-layer compositor: combine layers into the final frame buffer.

use crate::layer_manager::LayerManager;
use crate::damage_tracking::DamageTracker;
use ltk_core::geometry::Rect;

pub struct Compositor {
    pub layer_manager: LayerManager,
    pub damage:        DamageTracker,
}

impl Compositor {
    pub fn new() -> Self { Self { layer_manager: LayerManager::new(), damage: DamageTracker::new() } }

    /// Called by the frame loop: composite all layers into the backend surface.
    pub fn composite(&mut self) -> Vec<Rect> {
        self.damage.optimise();
        let dirty = self.damage.rects().to_vec();
        self.damage.clear();
        dirty
    }
}

impl Default for Compositor { fn default() -> Self { Self::new() } }
