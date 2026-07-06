//! Dock layout: top/bottom/left/right fill zones.
use ltk_core::geometry::{Rect, Size, SizeConstraint, Insets};
use crate::node::{LayoutTree, LayoutNodeId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DockSide { Top, Bottom, Left, Right, Fill }

pub struct DockLayout;

impl DockLayout {
    pub fn arrange(&self, tree: &mut LayoutTree, node_id: LayoutNodeId, available: Rect) {
        if let Some(n) = tree.get_mut(node_id) { n.rect = available; }
        // A real implementation would iterate children tagged with DockSide
        // and carve out regions in order (WinForms DockLayout style).
    }
}
