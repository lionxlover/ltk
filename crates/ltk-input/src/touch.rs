//! Multi-touch event types.

use ltk_core::geometry::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TouchId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TouchPhase { Started, Moved, Ended, Cancelled }

#[derive(Debug, Clone)]
pub struct TouchPoint {
    pub id:       TouchId,
    pub position: Point,
    pub pressure: f32,    // 0.0–1.0
    pub radius:   f32,    // contact area radius in logical px
}

#[derive(Debug, Clone)]
pub struct TouchEvent {
    pub phase:     TouchPhase,
    pub points:    Vec<TouchPoint>,
    pub timestamp: u64,
}

impl TouchEvent {
    pub fn primary(&self) -> Option<&TouchPoint> { self.points.first() }
    pub fn count(&self) -> usize { self.points.len() }
}
