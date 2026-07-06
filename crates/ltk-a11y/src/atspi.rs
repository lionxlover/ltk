//! AT-SPI2 D-Bus bridge.

use crate::tree::AccessibleNode;

/// Emits AT-SPI2 D-Bus events when the accessibility tree changes.
pub struct AtSpiAdapter {
    pub connected: bool,
}

impl AtSpiAdapter {
    pub fn new() -> Self {
        // Real impl: connect to the D-Bus session bus and register the app object.
        let connected = std::env::var("DISPLAY").is_ok() || std::env::var("WAYLAND_DISPLAY").is_ok();
        Self { connected }
    }

    /// Emit `object:property-change:accessible-name` (and others).
    pub fn emit_property_change(&self, node: &AccessibleNode) {
        if !self.connected { return; }
        log::trace!("AT-SPI2: property-change for {:?}", node.id);
        // Real impl: call atspi crate methods here.
    }

    /// Emit `object:children-changed:add`.
    pub fn emit_child_added(&self, parent: ltk_core::id::WidgetId, child: ltk_core::id::WidgetId) {
        log::trace!("AT-SPI2: child-added {:?} → {:?}", parent, child);
    }

    /// Emit `object:state-changed`.
    pub fn emit_state_change(&self, node: &AccessibleNode, state_name: &str, value: bool) {
        log::trace!("AT-SPI2: state-changed [{:?}] {} = {}", node.id, state_name, value);
    }

    /// Emit `focus:` event for screen readers.
    pub fn emit_focus(&self, node: &AccessibleNode) {
        log::debug!("AT-SPI2: focus → {:?} ({:?})", node.id, node.role);
    }
}

impl Default for AtSpiAdapter { fn default() -> Self { Self::new() } }
