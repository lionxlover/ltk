//! Mouse/pointer event types.

use ltk_core::geometry::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton { Left, Right, Middle, Back, Forward, Other(u8) }

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScrollDelta {
    pub dx: f32,
    pub dy: f32,
    pub is_pixel: bool,   // false = line-based scroll
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseEventKind {
    Moved, Pressed, Released, Clicked, DoubleClicked,
    Entered, Exited, Scrolled,
}

#[derive(Debug, Clone)]
pub struct MouseEvent {
    pub kind:      MouseEventKind,
    pub position:  Point,      // logical px, relative to window
    pub button:    Option<MouseButton>,
    pub delta:     Option<ScrollDelta>,
    pub modifiers: crate::keyboard::Modifiers,
    pub timestamp: u64,
}

impl MouseEvent {
    pub fn is_left_click(&self) -> bool {
        self.kind == MouseEventKind::Clicked && self.button == Some(MouseButton::Left)
    }
    pub fn is_right_click(&self) -> bool {
        self.kind == MouseEventKind::Clicked && self.button == Some(MouseButton::Right)
    }
}
