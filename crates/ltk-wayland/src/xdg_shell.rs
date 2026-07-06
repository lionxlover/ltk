//! XDG Shell protocol: toplevel windows, popups, configure events.

use ltk_core::geometry::Size;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XdgState { Maximized, Fullscreen, Resizing, Activated, TiledLeft, TiledRight, TiledTop, TiledBottom }

#[derive(Debug, Clone)]
pub struct XdgConfigureEvent {
    pub size:   Option<Size>,
    pub states: Vec<XdgState>,
}

/// Represents one `xdg_surface` + `xdg_toplevel` pair.
pub struct XdgSurface {
    pub configured: bool,
    pub pending_size: Option<Size>,
}

impl XdgSurface {
    pub fn new() -> Self { Self { configured: false, pending_size: None } }

    pub fn handle_configure(&mut self, event: XdgConfigureEvent) {
        self.pending_size = event.size;
        self.configured = true;
        log::debug!("XDG configure: size={:?} states={:?}", event.size, event.states);
    }

    pub fn ack_configure(&self, _serial: u32) {
        // Real impl: xdg_surface.ack_configure(serial)
    }

    pub fn set_title(&self, title: &str) {
        log::debug!("XDG: set_title({title:?})");
    }

    pub fn set_min_size(&self, size: Size) {
        log::debug!("XDG: set_min_size({size:?})");
    }
}

impl Default for XdgSurface { fn default() -> Self { Self::new() } }
