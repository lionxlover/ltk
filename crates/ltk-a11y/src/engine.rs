//! Master accessibility controller.

use crate::tree::AccessibilityTree;
use crate::atspi::AtSpiAdapter;
use crate::announcer::LiveAnnouncer;
use ltk_core::id::WidgetId;

pub struct A11yEngine {
    pub tree:      AccessibilityTree,
    pub adapter:   AtSpiAdapter,
    pub announcer: LiveAnnouncer,
    pub enabled:   bool,
}

impl A11yEngine {
    pub fn new(enabled: bool) -> Self {
        Self {
            tree:      AccessibilityTree::new(),
            adapter:   AtSpiAdapter::new(),
            announcer: LiveAnnouncer::new(),
            enabled,
        }
    }

    /// Sync dirty tree nodes to AT-SPI2. Call once per frame.
    pub fn flush(&mut self) {
        if !self.enabled { return; }
        let dirty = self.tree.drain_dirty();
        for id in &dirty {
            if let Some(node) = self.tree.get(*id) {
                self.adapter.emit_property_change(node);
            }
        }
    }

    /// Announce a message to screen readers (ARIA live region equivalent).
    pub fn announce(&self, text: &str, priority: crate::announcer::AnnouncePriority) {
        if self.enabled { self.announcer.announce(text, priority); }
    }
}
