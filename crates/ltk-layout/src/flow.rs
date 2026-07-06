//! Block/inline flow layout.
use ltk_core::geometry::{Rect, Size, SizeConstraint};
use crate::node::{LayoutTree, LayoutNodeId};
use crate::engine::LayoutPass;

pub struct FlowLayout;

impl LayoutPass for FlowLayout {
    fn measure(&self, tree: &mut LayoutTree, node_id: LayoutNodeId, c: SizeConstraint) -> Size {
        super::engine::LayoutEngine::new().measure_node(tree, node_id, c)
    }
    fn arrange(&self, tree: &mut LayoutTree, node_id: LayoutNodeId, available: Rect) {
        super::engine::LayoutEngine::new().arrange_node(tree, node_id, available);
    }
}
