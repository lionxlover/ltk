//! Platform-agnostic window trait.

use ltk_core::geometry::{Size, Rect, Point};
use ltk_core::id::WindowId;
use ltk_core::string::SharedString;

/// Logical window state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState { Normal, Minimized, Maximized, Fullscreen, Hidden }

/// Initial window configuration.
#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub title:       SharedString,
    pub size:        Size,
    pub min_size:    Option<Size>,
    pub max_size:    Option<Size>,
    pub position:    Option<Point>,
    pub resizable:   bool,
    pub decorations: bool,        // OS window chrome; false for custom titlebar
    pub transparent: bool,        // Requires compositor support
    pub always_on_top: bool,
    pub modal_for:   Option<WindowId>,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title:       SharedString::new("LTK Window"),
            size:        Size::new(1024.0, 768.0),
            min_size:    None,
            max_size:    None,
            position:    None,
            resizable:   true,
            decorations: false,   // LTK draws its own titlebar
            transparent: false,
            always_on_top: false,
            modal_for:   None,
        }
    }
}

/// Platform-agnostic window interface.
pub trait LtkWindow: Send + Sync {
    fn id(&self) -> WindowId;
    fn title(&self) -> &str;
    fn set_title(&self, title: &str);
    fn size(&self) -> Size;
    fn set_size(&self, size: Size);
    fn position(&self) -> Point;
    fn set_position(&self, pos: Point);
    fn state(&self) -> WindowState;
    fn set_state(&self, state: WindowState);
    fn is_focused(&self) -> bool;
    fn request_focus(&self);
    fn scale_factor(&self) -> f32;
    fn close(&self);
    fn show(&self);
    fn hide(&self);
    fn physical_size(&self) -> (u32, u32) {
        let s = self.size();
        let f = self.scale_factor();
        ((s.width * f) as u32, (s.height * f) as u32)
    }
}
