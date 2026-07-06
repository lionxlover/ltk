//! Monitor enumeration and geometry.

use ltk_core::geometry::{Size, Rect};
use ltk_core::id::MonitorId;

#[derive(Debug, Clone)]
pub struct Monitor {
    pub id:           MonitorId,
    pub name:         String,
    pub bounds:       Rect,         // logical px
    pub physical_bounds: Rect,      // physical px
    pub scale_factor: f32,
    pub refresh_hz:   f32,
    pub is_primary:   bool,
    pub color_depth:  u8,
    pub supports_hdr: bool,
}

impl Monitor {
    pub fn logical_size(&self)  -> Size { self.bounds.size }
    pub fn physical_size(&self) -> Size { self.physical_bounds.size }
    pub fn work_area(&self) -> Rect {
        // Subtract taskbar/panel. Real impl reads from compositor.
        Rect::new(self.bounds.x(), self.bounds.y() + 28.0,
                  self.bounds.width(), self.bounds.height() - 28.0)
    }
}

/// Enumerates physical displays.
pub struct MonitorManager { pub monitors: Vec<Monitor> }

impl MonitorManager {
    pub fn new() -> Self { Self { monitors: Vec::new() } }

    pub fn primary(&self) -> Option<&Monitor> {
        self.monitors.iter().find(|m| m.is_primary)
    }
    pub fn for_point(&self, p: ltk_core::geometry::Point) -> Option<&Monitor> {
        self.monitors.iter().find(|m| m.bounds.contains(p))
    }
    pub fn count(&self) -> usize { self.monitors.len() }

    /// Add a fake primary monitor (used in tests and offscreen mode).
    pub fn add_primary_1080p(&mut self) {
        self.monitors.push(Monitor {
            id:               MonitorId::new(),
            name:             "Primary".into(),
            bounds:           Rect::new(0.0, 0.0, 1920.0, 1080.0),
            physical_bounds:  Rect::new(0.0, 0.0, 1920.0, 1080.0),
            scale_factor:     1.0,
            refresh_hz:       60.0,
            is_primary:       true,
            color_depth:      24,
            supports_hdr:     false,
        });
    }
}

impl Default for MonitorManager { fn default() -> Self { Self::new() } }
