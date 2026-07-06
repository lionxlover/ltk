//! Central input manager: device registry and event routing.

use crate::keyboard::KeyEvent;
use crate::mouse::MouseEvent;
use crate::touch::TouchEvent;
use crate::gesture::GestureEvent;
use ltk_core::id::WidgetId;

/// Routes normalized input events to the focused/hovered widget.
pub struct InputManager {
    pub focus:    Option<WidgetId>,
    pub hover:    Option<WidgetId>,
}

impl InputManager {
    pub fn new() -> Self { Self { focus: None, hover: None } }

    /// Deliver a key event to the focused widget (returns true if consumed).
    pub fn dispatch_key(&self, _event: &KeyEvent) -> bool { false }

    /// Deliver a mouse event, updating hover state.
    pub fn dispatch_mouse(&mut self, event: &MouseEvent) -> bool { false }

    /// Deliver a touch event.
    pub fn dispatch_touch(&mut self, _event: &TouchEvent) -> bool { false }
}

impl Default for InputManager { fn default() -> Self { Self::new() } }
