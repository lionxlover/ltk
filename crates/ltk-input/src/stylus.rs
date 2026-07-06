//! Stylus / drawing tablet input.

use ltk_core::geometry::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StylusButton { Barrel1, Barrel2, Eraser, Tip }

#[derive(Debug, Clone)]
pub struct StylusEvent {
    pub position:  Point,
    pub pressure:  f32,        // 0.0–1.0
    pub tilt_x:    f32,        // degrees −90 to +90
    pub tilt_y:    f32,
    pub twist:     f32,        // pen rotation 0–360
    pub in_range:  bool,
    pub button:    Option<StylusButton>,
    pub timestamp: u64,
}
