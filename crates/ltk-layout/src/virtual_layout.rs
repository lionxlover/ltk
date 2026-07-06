//! Virtualized list/grid layout — only measure visible items.
use ltk_core::geometry::{Rect, Size};

/// A virtualized item descriptor (index + cached height).
#[derive(Debug, Clone, Copy)]
pub struct VirtualItem {
    pub index:  usize,
    pub height: f32,    // estimated / cached
    pub y:      f32,    // absolute Y offset in the list
}

/// Computes which items are visible and their positions for virtualized scroll.
pub struct VirtualLayout {
    pub item_height:    Option<f32>,    // None = variable height
    pub item_count:     usize,
    pub scroll_offset:  f32,
    pub viewport_height:f32,
    item_heights:       Vec<f32>,
    item_offsets:       Vec<f32>,
}

impl VirtualLayout {
    pub fn new(item_count: usize, default_height: f32) -> Self {
        let item_heights = vec![default_height; item_count];
        let mut offsets  = Vec::with_capacity(item_count + 1);
        let mut y = 0.0f32;
        offsets.push(0.0);
        for &h in &item_heights { y += h; offsets.push(y); }
        Self {
            item_height: Some(default_height),
            item_count,
            scroll_offset: 0.0,
            viewport_height: 600.0,
            item_heights,
            item_offsets: offsets,
        }
    }

    /// Returns the range of visible item indices.
    pub fn visible_range(&self) -> std::ops::Range<usize> {
        if self.item_count == 0 { return 0..0; }
        let start = self.item_offsets.partition_point(|&y| y < self.scroll_offset)
            .saturating_sub(1);
        let end = self.item_offsets.partition_point(|&y| y < self.scroll_offset + self.viewport_height)
            .min(self.item_count);
        start..end
    }

    /// Get the visual Rect for a given item index within the viewport.
    pub fn item_rect(&self, index: usize, viewport_x: f32, viewport_width: f32) -> Rect {
        let y = self.item_offsets.get(index).copied().unwrap_or(0.0)
              - self.scroll_offset;
        let h = self.item_heights.get(index).copied().unwrap_or(0.0);
        Rect::new(viewport_x, y, viewport_width, h)
    }

    /// Update the measured height of an item and recompute offsets.
    pub fn set_item_height(&mut self, index: usize, height: f32) {
        if index >= self.item_heights.len() { return; }
        self.item_heights[index] = height;
        let mut y = if index > 0 { self.item_offsets[index] } else { 0.0 };
        for i in index..self.item_heights.len() {
            self.item_offsets[i] = y;
            y += self.item_heights[i];
        }
        if let Some(last) = self.item_offsets.last_mut() { *last = y; }
    }

    /// Total scrollable height.
    pub fn total_height(&self) -> f32 {
        self.item_offsets.last().copied().unwrap_or(0.0)
    }

    /// Set the scroll position (clamped to valid range).
    pub fn set_scroll(&mut self, offset: f32) {
        self.scroll_offset = offset.clamp(0.0, (self.total_height() - self.viewport_height).max(0.0));
    }
}
