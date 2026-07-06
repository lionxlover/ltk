//! wl_output: physical display geometry and mode reporting.

use ltk_core::geometry::{Point, Size};

#[derive(Debug, Clone)]
pub struct WaylandOutput {
    pub name:    String,
    pub position:Point,
    pub size:    Size,         // physical mm
    pub mode_px: (u32, u32),
    pub refresh_mhz: u32,      // milli-Hz per Wayland spec
    pub scale:   i32,
}

impl WaylandOutput {
    pub fn refresh_hz(&self) -> f32 { self.refresh_mhz as f32 / 1000.0 }
}
