//! # ltk-core
//!
//! Shared primitive types for the entire LTK framework.
//! Every other crate depends on this one; it must never grow large.
//!
//! ## Modules
//! - [`id`] — typed IDs for every framework entity
//! - [`geometry`] — Point, Size, Rect, Insets, transforms
//! - [`color`] — Color in linear sRGB with OKLCH conversion
//! - [`string`] — SharedString, SmallString
//! - [`error`] — LtkError, LtkResult
//! - [`sync`] — framework-aware mutex/rwlock wrappers
//! - [`task`] — async executor interface
//! - [`time`] — Duration, Instant, FrameTime
//! - [`version`] — SemVer, compatibility
//! - [`flags`] — bitflag helpers
//! - [`callback`] — type-erased callable storage
//! - [`env`] — runtime environment detection
//! - [`arena`] — typed arena allocators
//! - [`slot_map`] — component storage

#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::all)]

pub mod id;
pub mod arena;
pub mod slot_map;
pub mod geometry;
pub mod color;
pub mod string;
pub mod error;
pub mod sync;
pub mod task;
pub mod time;
pub mod version;
pub mod flags;
pub mod callback;
pub mod env;

pub use id::*;
pub use geometry::{Point, Size, Rect, Insets, Transform2D};
pub use color::Color;
pub use string::SharedString;
pub use error::{LtkError, LtkResult};
pub use time::{Duration, Instant, FrameTime};
pub use version::Version;
