//! Core layout engine: orchestrates measure + arrange passes.

use ltk_core::geometry::{Rect, Size, SizeConstraint};
use crate::node::{LayoutNodeId, LayoutTree, LayoutMode};
use crate::flex::FlexLayout;
use crate::grid::GridLayout;

/// Trait implemented by each layout algorithm.
pub trait LayoutPass: Send + Sync {
    /// Compute the intrinsic/constrained size of a node given parent constraints.
    fn measure(
        &self,
        tree:       &mut LayoutTree,
        node_id:    LayoutNodeId,
        constraint: SizeConstraint,
    ) -> Size;

    /// Assign final `Rect` to this node and all descendants within `available`.
    fn arrange(
        &self,
        tree:       &mut LayoutTree,
        node_id:    LayoutNodeId,
        available:  Rect,
    );
}

/// Central layout engine for one layout tree.
pub struct LayoutEngine {
    pub flex: FlexLayout,
    pub grid: GridLayout,
}

impl LayoutEngine {
    pub fn new() -> Self {
        Self { flex: FlexLayout, grid: GridLayout }
    }

    /// Run a full layout pass starting from the root node.
    pub fn layout(&self, tree: &mut LayoutTree, viewport: Rect) {
        let root = match tree.root() { Some(r) => r, None => return };
        let constraint = SizeConstraint::tight(viewport.size);
        self.measure_node(tree, root, constraint);
        self.arrange_node(tree, root, viewport);
    }

    pub fn measure_node(
        &self,
        tree:       &mut LayoutTree,
        node_id:    LayoutNodeId,
        constraint: SizeConstraint,
    ) -> Size {
        let mode = tree.get(node_id).map(|n| n.config.mode.clone()).unwrap_or_default();
        match mode {
            LayoutMode::Flex(_) => self.flex.measure(tree, node_id, constraint),
            LayoutMode::Grid(_) => self.grid.measure(tree, node_id, constraint),
            LayoutMode::Leaf   => self.measure_leaf(tree, node_id, constraint),
            LayoutMode::Fill | LayoutMode::Stack | LayoutMode::Flow =>
                self.measure_default(tree, node_id, constraint),
        }
    }

    pub fn arrange_node(
        &self,
        tree:       &mut LayoutTree,
        node_id:    LayoutNodeId,
        available:  Rect,
    ) {
        let mode = tree.get(node_id).map(|n| n.config.mode.clone()).unwrap_or_default();
        match mode {
            LayoutMode::Flex(_) => self.flex.arrange(tree, node_id, available),
            LayoutMode::Grid(_) => self.grid.arrange(tree, node_id, available),
            LayoutMode::Fill | LayoutMode::Stack =>
                self.arrange_fill(tree, node_id, available),
            _ => self.arrange_default(tree, node_id, available),
        }
        if let Some(node) = tree.get_mut(node_id) { node.dirty = false; }
    }

    fn measure_leaf(
        &self,
        tree:       &mut LayoutTree,
        node_id:    LayoutNodeId,
        constraint: SizeConstraint,
    ) -> Size {
        // In a real impl, call the widget's intrinsic_size() callback.
        // Placeholder: return preferred or zero.
        let preferred = tree.get(node_id)
            .and_then(|n| n.config.preferred_size)
            .unwrap_or(Size::ZERO);
        constraint.clamp(preferred)
    }

    fn measure_default(
        &self,
        tree:       &mut LayoutTree,
        node_id:    LayoutNodeId,
        constraint: SizeConstraint,
    ) -> Size {
        // Measure all children; compute bounding box.
        let children: Vec<_> = tree.get(node_id)
            .map(|n| n.children.to_vec())
            .unwrap_or_default();
        let mut total = Size::ZERO;
        for child in children {
            let s = self.measure_node(tree, child, constraint);
            total.width  = total.width.max(s.width);
            total.height += s.height;
        }
        constraint.clamp(total)
    }

    fn arrange_fill(
        &self,
        tree:       &mut LayoutTree,
        node_id:    LayoutNodeId,
        available:  Rect,
    ) {
        if let Some(node) = tree.get_mut(node_id) { node.rect = available; }
        let children: Vec<_> = tree.get(node_id)
            .map(|n| n.children.to_vec())
            .unwrap_or_default();
        for child in children { self.arrange_node(tree, child, available); }
    }

    fn arrange_default(
        &self,
        tree:       &mut LayoutTree,
        node_id:    LayoutNodeId,
        available:  Rect,
    ) {
        if let Some(node) = tree.get_mut(node_id) { node.rect = available; }
        let children: Vec<_> = tree.get(node_id)
            .map(|n| n.children.to_vec())
            .unwrap_or_default();
        let mut y = available.y();
        for child in children {
            let h = tree.get(child).map(|n| n.rect.height()).unwrap_or(0.0);
            let r = Rect::new(available.x(), y, available.width(), h);
            self.arrange_node(tree, child, r);
            y += h;
        }
    }
}

impl Default for LayoutEngine { fn default() -> Self { Self::new() } }
