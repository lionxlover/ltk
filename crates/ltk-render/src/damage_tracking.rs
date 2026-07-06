//! Track damaged (dirty) regions to minimize repaint area.

use ltk_core::geometry::Rect;

/// Accumulates damaged rects and merges them into a minimal repaint set.
pub struct DamageTracker {
    rects: Vec<Rect>,
    frame_rect: Option<Rect>,  // bounding box of all damage this frame
}

impl DamageTracker {
    pub fn new() -> Self { Self { rects: Vec::new(), frame_rect: None } }

    /// Mark a region as dirty (will be repainted this frame).
    pub fn add(&mut self, rect: Rect) {
        if rect.is_empty() { return; }
        self.frame_rect = Some(match self.frame_rect {
            Some(r) => r.union(rect),
            None    => rect,
        });
        self.rects.push(rect);
    }

    pub fn is_dirty(&self) -> bool { !self.rects.is_empty() }
    pub fn bounding_box(&self) -> Option<Rect> { self.frame_rect }
    pub fn rects(&self) -> &[Rect] { &self.rects }

    /// Clear at end of frame.
    pub fn clear(&mut self) { self.rects.clear(); self.frame_rect = None; }

    /// Merge overlapping rects into a smaller set (greedy).
    pub fn optimise(&mut self) {
        if self.rects.len() <= 2 { return; }
        // Simplified: replace with bounding box if there are many small rects
        if self.rects.len() > 8 {
            if let Some(bb) = self.frame_rect {
                self.rects = vec![bb];
            }
        }
    }
}

impl Default for DamageTracker { fn default() -> Self { Self::new() } }
