//! Resizable split-pane layout.
use ltk_core::geometry::{Rect, Size};
use crate::node::{LayoutTree, LayoutNodeId};

pub struct SplitLayout;
impl SplitLayout {
    pub fn arrange(&self, tree: &mut LayoutTree, node_id: LayoutNodeId, available: Rect, ratio: f32, vertical: bool) {
        if let Some(n) = tree.get_mut(node_id) { n.rect = available; }
        let children: Vec<_> = tree.get(node_id).map(|n| n.children.to_vec()).unwrap_or_default();
        if children.len() < 2 { return; }
        let (ra, rb) = if vertical {
            let ha = available.height() * ratio;
            (Rect::new(available.x(), available.y(), available.width(), ha),
             Rect::new(available.x(), available.y() + ha, available.width(), available.height() - ha))
        } else {
            let wa = available.width() * ratio;
            (Rect::new(available.x(), available.y(), wa, available.height()),
             Rect::new(available.x() + wa, available.y(), available.width() - wa, available.height()))
        };
        let engine = crate::engine::LayoutEngine::new();
        engine.arrange_node(tree, children[0], ra);
        engine.arrange_node(tree, children[1], rb);
    }
}
