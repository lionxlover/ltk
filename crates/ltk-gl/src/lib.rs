//! # ltk-gl — OpenGL 4.6 / OpenGL ES 3.2 Rendering Backend
//!
//! Implements [`ltk_render::backend_api::RenderBackend`] using OpenGL.
//! Targets desktop GL 4.6 with an ES 3.2 fallback path for embedded/Wayland
//! compositors that only expose EGL/GLES contexts.

pub mod context;
pub mod shader;
pub mod backend;

pub use backend::GlBackend;
pub use context::GlContext;
