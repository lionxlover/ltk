//! Stack (Z) layout — children overlay each other.
use ltk_core::geometry::Rect;
use crate::node::{LayoutTree, LayoutNodeId};
use crate::engine::LayoutEngine;

pub struct StackLayout;
impl StackLayout {
    pub fn arrange(&self, engine: &LayoutEngine, tree: &mut LayoutTree, node_id: LayoutNodeId, available: Rect) {
        if let Some(n) = tree.get_mut(node_id) { n.rect = available; }
        let children: Vec<_> = tree.get(node_id).map(|n| n.children.to_vec()).unwrap_or_default();
        for child in children { engine.arrange_node(tree, child, available); }
    }
}
