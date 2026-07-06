//! # ltk-a11y — AT-SPI2 · Accessibility Tree · Screen Reader · Focus Nav

pub mod engine;
pub mod tree;
pub mod atspi;
pub mod keyboard_nav;
pub mod high_contrast;
pub mod announcer;

pub use engine::A11yEngine;
pub use tree::{AccessibleNode, AccessibleRole, AccessibleState, AccessibleAction};
pub use atspi::AtSpiAdapter;
