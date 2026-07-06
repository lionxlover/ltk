//! Full Flexbox layout algorithm (CSS Flexible Box Module Level 1).

use ltk_core::geometry::{Rect, Size, SizeConstraint, Insets};
use crate::node::{LayoutTree, LayoutNodeId, LayoutMode};
use crate::engine::LayoutPass;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FlexDirection { #[default] Row, RowReverse, Column, ColumnReverse }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FlexWrap { #[default] NoWrap, Wrap, WrapReverse }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlignItems { Stretch, #[default] Start, Center, End, Baseline }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlignSelf  { Auto, Stretch, #[default] Start, Center, End, Baseline }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum JustifyContent { #[default] Start, Center, End, SpaceBetween, SpaceAround, SpaceEvenly }

/// Flexbox container configuration.
#[derive(Debug, Clone, Default)]
pub struct FlexConfig {
    pub direction:     FlexDirection,
    pub wrap:          FlexWrap,
    pub align_items:   AlignItems,
    pub align_content: AlignItems,
    pub justify:       JustifyContent,
    pub gap:           f32,
    pub row_gap:       f32,
    pub col_gap:       f32,
}

impl FlexConfig {
    pub fn row() -> Self { Self { direction: FlexDirection::Row, ..Default::default() } }
    pub fn column() -> Self { Self { direction: FlexDirection::Column, ..Default::default() } }
    pub fn is_row(&self) -> bool {
        matches!(self.direction, FlexDirection::Row | FlexDirection::RowReverse)
    }
    pub fn main_axis_gap(&self) -> f32 {
        let g = if self.gap > 0.0 { self.gap } else { 0.0 };
        if self.is_row() { self.col_gap.max(g) } else { self.row_gap.max(g) }
    }
}

pub struct FlexLayout;

impl LayoutPass for FlexLayout {
    fn measure(
        &self,
        tree:       &mut LayoutTree,
        node_id:    LayoutNodeId,
        constraint: SizeConstraint,
    ) -> Size {
        let config = match tree.get(node_id).map(|n| n.config.mode.clone()) {
            Some(LayoutMode::Flex(c)) => c,
            _ => return Size::ZERO,
        };

        let children: Vec<_> = tree.get(node_id)
            .map(|n| n.children.to_vec())
            .unwrap_or_default();

        let gap = config.main_axis_gap();
        let mut main_total = 0.0f32;
        let mut cross_max  = 0.0f32;

        for (i, child) in children.iter().enumerate() {
            let child_size = super::engine::LayoutEngine::new()
                .measure_node(tree, *child, SizeConstraint::UNBOUNDED);
            if config.is_row() {
                main_total += child_size.width;
                cross_max   = cross_max.max(child_size.height);
            } else {
                main_total += child_size.height;
                cross_max   = cross_max.max(child_size.width);
            }
            if i < children.len().saturating_sub(1) { main_total += gap; }
        }

        let size = if config.is_row() {
            Size::new(main_total, cross_max)
        } else {
            Size::new(cross_max, main_total)
        };

        let pad = tree.get(node_id).map(|n| n.config.padding).unwrap_or(Insets::ZERO);
        let padded = Size::new(
            size.width  + pad.horizontal_sum(),
            size.height + pad.vertical_sum(),
        );
        constraint.clamp(padded)
    }

    fn arrange(
        &self,
        tree:       &mut LayoutTree,
        node_id:    LayoutNodeId,
        available:  Rect,
    ) {
        if let Some(n) = tree.get_mut(node_id) { n.rect = available; }

        let (config, children, padding) = match tree.get(node_id) {
            Some(n) => {
                let c = match &n.config.mode { LayoutMode::Flex(c) => c.clone(), _ => return };
                (c, n.children.to_vec(), n.config.padding)
            }
            None => return,
        };

        let content = available.inset(padding);
        let gap     = config.main_axis_gap();
        let n_ch    = children.len();
        if n_ch == 0 { return; }

        // Collect measured sizes
        let engine = super::engine::LayoutEngine::new();
        let sizes: Vec<Size> = children.iter()
            .map(|&c| engine.measure_node(tree, c, SizeConstraint::loose(content.size)))
            .collect();

        // Compute main-axis total and free space
        let total_main: f32 = sizes.iter().map(|s| {
            if config.is_row() { s.width } else { s.height }
        }).sum::<f32>() + gap * (n_ch.saturating_sub(1)) as f32;

        let available_main = if config.is_row() { content.width() } else { content.height() };
        let free  = (available_main - total_main).max(0.0);
        let cross = if config.is_row() { content.height() } else { content.width() };

        // Distribute free space per justify-content
        let (start_offset, between) = match config.justify {
            JustifyContent::Start        => (0.0, 0.0),
            JustifyContent::End          => (free, 0.0),
            JustifyContent::Center       => (free * 0.5, 0.0),
            JustifyContent::SpaceBetween => (0.0, if n_ch > 1 { free / (n_ch - 1) as f32 } else { 0.0 }),
            JustifyContent::SpaceAround  => {
                let each = free / n_ch as f32;
                (each * 0.5, each)
            }
            JustifyContent::SpaceEvenly  => {
                let each = free / (n_ch + 1) as f32;
                (each, each)
            }
        };

        // Place children
        let mut main_cursor = if config.is_row() {
            content.x() + start_offset
        } else {
            content.y() + start_offset
        };

        for (i, (&child, size)) in children.iter().zip(sizes.iter()).enumerate() {
            let cross_start = content.y();
            let child_cross = if config.is_row() { size.height } else { size.width };
            let cross_pos = match config.align_items {
                AlignItems::Center  => cross_start + (cross - child_cross) * 0.5,
                AlignItems::End     => cross_start + cross - child_cross,
                AlignItems::Stretch => cross_start,
                _                   => cross_start,
            };

            let rect = if config.is_row() {
                Rect::new(main_cursor, cross_pos, size.width, if config.align_items == AlignItems::Stretch { cross } else { size.height })
            } else {
                Rect::new(cross_pos, main_cursor, if config.align_items == AlignItems::Stretch { cross } else { size.width }, size.height)
            };

            engine.arrange_node(tree, child, rect);
            main_cursor += (if config.is_row() { size.width } else { size.height }) + gap + between;
        }
    }
}
