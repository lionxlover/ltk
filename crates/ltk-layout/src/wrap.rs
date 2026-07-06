//! Wrap/flow layout — children wrap to the next row/column.
use ltk_core::geometry::{Rect, Size, SizeConstraint};
use crate::node::{LayoutTree, LayoutNodeId};

pub struct WrapLayout;
impl WrapLayout {
    pub fn arrange(&self, tree: &mut LayoutTree, node_id: LayoutNodeId, available: Rect, gap: f32) {
        if let Some(n) = tree.get_mut(node_id) { n.rect = available; }
        let children: Vec<_> = tree.get(node_id).map(|n| n.children.to_vec()).unwrap_or_default();
        let engine = crate::engine::LayoutEngine::new();
        let mut x = available.x();
        let mut y = available.y();
        let mut row_h = 0.0f32;
        for child in children {
            let s = engine.measure_node(tree, child, SizeConstraint::loose(available.size));
            if x + s.width > available.max_x() && x > available.x() {
                x = available.x(); y += row_h + gap; row_h = 0.0;
            }
            engine.arrange_node(tree, child, Rect::new(x, y, s.width, s.height));
            x += s.width + gap;
            row_h = row_h.max(s.height);
        }
    }
}
