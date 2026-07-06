//! CSS Grid layout algorithm.

use ltk_core::geometry::{Rect, Size, SizeConstraint};
use crate::node::{LayoutTree, LayoutNodeId};
use crate::engine::LayoutPass;

#[derive(Debug, Clone, PartialEq)]
pub enum TrackSize {
    Fixed(f32),
    Fraction(f32),    // fr unit
    MinContent,
    MaxContent,
    Auto,
    MinMax(Box<TrackSize>, Box<TrackSize>),
}

impl TrackSize {
    pub fn resolve(&self, available: f32, total_fr: f32, free: f32) -> f32 {
        match self {
            Self::Fixed(px)    => *px,
            Self::Fraction(fr) => if total_fr > 0.0 { fr / total_fr * free } else { 0.0 },
            Self::Auto         => available,
            Self::MinContent | Self::MaxContent => available, // simplified
            Self::MinMax(_, max) => max.resolve(available, total_fr, free),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct GridArea {
    pub row_start:  i32,
    pub row_end:    i32,
    pub col_start:  i32,
    pub col_end:    i32,
}

#[derive(Debug, Clone, Default)]
pub struct GridConfig {
    pub columns:    Vec<TrackSize>,
    pub rows:       Vec<TrackSize>,
    pub col_gap:    f32,
    pub row_gap:    f32,
    pub areas:      Option<Vec<Vec<String>>>,  // named template areas
}

pub struct GridLayout;

impl LayoutPass for GridLayout {
    fn measure(
        &self,
        tree:       &mut LayoutTree,
        node_id:    LayoutNodeId,
        constraint: SizeConstraint,
    ) -> Size {
        // Full grid measurement would compute track sizes then sum.
        // Simplified: return preferred or available.
        let preferred = tree.get(node_id)
            .and_then(|n| n.config.preferred_size)
            .unwrap_or_else(|| constraint.max);
        constraint.clamp(preferred)
    }

    fn arrange(
        &self,
        tree:       &mut LayoutTree,
        node_id:    LayoutNodeId,
        available:  Rect,
    ) {
        if let Some(n) = tree.get_mut(node_id) { n.rect = available; }
        let children: Vec<_> = tree.get(node_id)
            .map(|n| n.children.to_vec())
            .unwrap_or_default();

        let (config, padding) = tree.get(node_id).map(|n| {
            if let crate::node::LayoutMode::Grid(c) = &n.config.mode {
                (c.clone(), n.config.padding)
            } else {
                (GridConfig::default(), n.config.padding)
            }
        }).unwrap_or_default();

        let content = available.inset(padding);
        if config.columns.is_empty() || children.is_empty() { return; }

        let ncols = config.columns.len();
        let col_gap_total = config.col_gap * (ncols.saturating_sub(1)) as f32;
        let available_w = content.width() - col_gap_total;
        let total_fr: f32 = config.columns.iter().map(|t| if let TrackSize::Fraction(f) = t { *f } else { 0.0 }).sum();

        // Resolve column widths
        let col_widths: Vec<f32> = config.columns.iter()
            .map(|t| t.resolve(available_w / ncols as f32, total_fr, available_w))
            .collect();

        let engine = super::engine::LayoutEngine::new();
        let mut col = 0usize;
        let mut row_y = content.y();
        let row_h = content.height() / ((children.len() + ncols - 1) / ncols).max(1) as f32;

        for child in children {
            let x = content.x() + col_widths[..col].iter().sum::<f32>() + col as f32 * config.col_gap;
            let rect = Rect::new(x, row_y, col_widths[col], row_h - config.row_gap);
            engine.arrange_node(tree, child, rect);
            col += 1;
            if col >= ncols { col = 0; row_y += row_h; }
        }
    }
}
