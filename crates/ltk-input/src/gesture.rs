//! High-level gesture recognizer events.

use ltk_core::geometry::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GestureKind {
    Tap, DoubleTap, LongPress,
    Pan, Pinch, Rotation, Swipe,
    EdgeSwipe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GestureState { Began, Changed, Ended, Cancelled }

#[derive(Debug, Clone)]
pub struct GestureEvent {
    pub kind:     GestureKind,
    pub state:    GestureState,
    pub center:   Point,    // centroid of all touch points
    pub delta:    Point,    // pan delta (for Pan)
    pub scale:    f32,      // pinch scale factor (for Pinch)
    pub rotation: f32,      // rotation in radians (for Rotation)
    pub velocity: Point,    // fling velocity px/s (for Swipe/Pan)
    pub timestamp:u64,
}

impl GestureEvent {
    pub fn is_fling(&self) -> bool {
        self.kind == GestureKind::Swipe &&
        (self.velocity.x.powi(2) + self.velocity.y.powi(2)).sqrt() > 200.0
    }
}
