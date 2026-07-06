//! Layout node: stores per-node layout configuration and results.

use ltk_core::{geometry::{Rect, Size, Insets, SizeConstraint}, id::WidgetId};
use smallvec::SmallVec;
use crate::flex::FlexConfig;
use crate::grid::GridConfig;

/// Unique identifier for a layout node (mirrors WidgetId 1:1).
pub type LayoutNodeId = WidgetId;

/// How a node participates in layout.
#[derive(Debug, Clone, PartialEq)]
pub enum LayoutMode {
    /// Children are arranged by the Flexbox algorithm.
    Flex(FlexConfig),
    /// Children are arranged on a CSS Grid.
    Grid(GridConfig),
    /// Normal block/inline flow.
    Flow,
    /// Children are stacked in Z order; absolute positioning.
    Stack,
    /// All children fill the full parent Rect (overlay).
    Fill,
    /// Leaf node: reports its own intrinsic size.
    Leaf,
}

impl Default for LayoutMode { fn default() -> Self { Self::Flow } }

/// Per-node layout configuration (set once; read during layout).
#[derive(Debug, Clone)]
pub struct LayoutConfig {
    pub mode:          LayoutMode,
    pub padding:       Insets,
    pub margin:        Insets,
    pub min_size:      Size,
    pub max_size:      Size,
    pub preferred_size:Option<Size>,
    pub flex_grow:     f32,
    pub flex_shrink:   f32,
    pub flex_basis:    Option<f32>,
    pub align_self:    Option<crate::flex::AlignSelf>,
    pub z_index:       i32,
    pub visible:       bool,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            mode:           LayoutMode::default(),
            padding:        Insets::ZERO,
            margin:         Insets::ZERO,
            min_size:       Size::ZERO,
            max_size:       Size::INFINITY,
            preferred_size: None,
            flex_grow:      0.0,
            flex_shrink:    1.0,
            flex_basis:     None,
            align_self:     None,
            z_index:        0,
            visible:        true,
        }
    }
}

/// A node in the layout tree.
pub struct LayoutNode {
    pub id:       LayoutNodeId,
    pub config:   LayoutConfig,
    pub children: SmallVec<[LayoutNodeId; 8]>,
    pub parent:   Option<LayoutNodeId>,
    // Output of layout passes:
    pub rect:     Rect,          // final position + size (logical px)
    pub baseline: f32,           // distance from top to text baseline
    pub dirty:    bool,          // needs re-layout?
}

impl LayoutNode {
    pub fn new(id: LayoutNodeId) -> Self {
        Self {
            id, config: LayoutConfig::default(),
            children: SmallVec::new(), parent: None,
            rect: Rect::ZERO, baseline: 0.0, dirty: true,
        }
    }
}

/// The complete layout tree for one window.
pub struct LayoutTree {
    nodes: std::collections::HashMap<LayoutNodeId, LayoutNode>,
    root:  Option<LayoutNodeId>,
}

impl LayoutTree {
    pub fn new() -> Self { Self { nodes: Default::default(), root: None } }

    pub fn insert(&mut self, node: LayoutNode) {
        self.nodes.insert(node.id, node);
    }

    pub fn root(&self) -> Option<LayoutNodeId> { self.root }
    pub fn set_root(&mut self, id: LayoutNodeId) { self.root = Some(id); }

    pub fn get(&self, id: LayoutNodeId) -> Option<&LayoutNode> { self.nodes.get(&id) }
    pub fn get_mut(&mut self, id: LayoutNodeId) -> Option<&mut LayoutNode> { self.nodes.get_mut(&id) }

    pub fn mark_dirty(&mut self, id: LayoutNodeId) {
        if let Some(n) = self.nodes.get_mut(&id) { n.dirty = true; }
        // Bubble up to root
        let mut cur = id;
        while let Some(parent) = self.nodes.get(&cur).and_then(|n| n.parent) {
            if let Some(pn) = self.nodes.get_mut(&parent) {
                if pn.dirty { break; }
                pn.dirty = true;
            }
            cur = parent;
        }
    }

    pub fn rect(&self, id: LayoutNodeId) -> Option<Rect> {
        self.nodes.get(&id).map(|n| n.rect)
    }

    pub fn iter(&self) -> impl Iterator<Item = &LayoutNode> { self.nodes.values() }
}

impl Default for LayoutTree { fn default() -> Self { Self::new() } }
