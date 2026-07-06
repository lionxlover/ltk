//! RenderBackend trait: abstract over OpenGL and Vulkan.

use ltk_core::geometry::{Size, Rect};

/// Capabilities reported by the active backend.
#[derive(Debug, Clone)]
pub struct BackendCapabilities {
    pub name:           String,
    pub max_texture_size: u32,
    pub supports_msaa:  bool,
    pub supports_hdr:   bool,
    pub supports_blend_modes: bool,
}

/// Per-frame context passed between begin and end.
pub struct FrameContext { pub frame_nr: u64 }

/// Abstraction over all GPU rendering backends.
pub trait RenderBackend: Send + Sync {
    fn name(&self) -> &str;
    fn capabilities(&self) -> BackendCapabilities;
    fn begin_frame(&mut self) -> FrameContext;
    fn end_frame(&mut self, ctx: FrameContext);
    fn present(&mut self);
    fn resize(&mut self, physical_size: Size);
    fn viewport(&self) -> Size;
}
