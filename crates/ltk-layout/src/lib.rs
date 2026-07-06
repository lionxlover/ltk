//! # ltk-layout
//! Complete layout engine for LTK.
//!
//! Layout happens in two passes:
//! 1. **Measure** — query every node for its intrinsic/constrained size
//! 2. **Arrange** — assign a final `Rect` to every node
//!
//! All algorithms share the same `LayoutNode` representation and
//! `LayoutEngine` trait.

#![warn(missing_docs)]

pub mod node;
pub mod engine;
pub mod flex;
pub mod grid;
pub mod flow;
pub mod anchor;
pub mod dock;
pub mod stack;
pub mod split;
pub mod wrap;
pub mod responsive;
pub mod virtual_layout;

pub use node::{LayoutNode, LayoutNodeId, LayoutTree};
pub use engine::{LayoutEngine, LayoutPass};
pub use flex::{FlexConfig, FlexDirection, FlexWrap, AlignItems, JustifyContent};
pub use grid::{GridConfig, TrackSize, GridArea};
pub use responsive::{Breakpoint, BreakpointSet, ResponsiveRule};
pub use virtual_layout::{VirtualLayout, VirtualItem};
