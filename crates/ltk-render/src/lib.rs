//! # ltk-render — Scene Graph · Compositor · Canvas · GPU Backends

pub mod scene_graph;
pub mod layer_manager;
pub mod surface_manager;
pub mod frame_scheduler;
pub mod dirty_tracking;
pub mod damage_tracking;
pub mod compositor;
pub mod canvas;
pub mod paint;
pub mod vector;
pub mod texture_manager;
pub mod glyph_cache;
pub mod image_renderer;
pub mod backend_api;

pub use scene_graph::{SceneGraph, SceneNode, SceneNodeId};
pub use canvas::{Canvas, DrawCommand};
pub use paint::{Paint, Gradient, GradientStop};
pub use backend_api::{RenderBackend, BackendCapabilities, FrameContext};
pub use texture_manager::{TextureManager, TextureId};
pub use damage_tracking::DamageTracker;
