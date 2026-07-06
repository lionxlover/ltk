//! Anchor/constraint-based layout (Auto Layout style).
use ltk_core::geometry::{Rect, Size, SizeConstraint};
use crate::node::{LayoutTree, LayoutNodeId};

pub struct AnchorLayout;
// Full implementation would embed a Cassowary solver.
// Placeholder: passes through to stack layout.
impl AnchorLayout {
    pub fn measure(&self, _tree: &mut LayoutTree, _id: LayoutNodeId, _c: SizeConstraint) -> Size {
        Size::ZERO
    }
    pub fn arrange(&self, tree: &mut LayoutTree, id: LayoutNodeId, available: Rect) {
        if let Some(n) = tree.get_mut(id) { n.rect = available; }
    }
}
