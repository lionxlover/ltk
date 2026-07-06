//! GlBackend: RenderBackend implementation over OpenGL.

use ltk_render::backend_api::{RenderBackend, BackendCapabilities, FrameContext};
use ltk_core::geometry::Size;
use crate::context::GlContext;

pub struct GlBackend {
    ctx:      GlContext,
    viewport: Size,
    frame_nr: u64,
}

impl GlBackend {
    pub fn new(vsync: bool) -> ltk_core::LtkResult<Self> {
        Ok(Self { ctx: GlContext::create(vsync)?, viewport: Size::ZERO, frame_nr: 0 })
    }
}

impl RenderBackend for GlBackend {
    fn name(&self) -> &str { "OpenGL 4.6" }

    fn capabilities(&self) -> BackendCapabilities {
        BackendCapabilities {
            name: "OpenGL 4.6 / ES 3.2".into(),
            max_texture_size: 16384,
            supports_msaa: true,
            supports_hdr: false,
            supports_blend_modes: true,
        }
    }

    fn begin_frame(&mut self) -> FrameContext {
        self.frame_nr += 1;
        let _ = self.ctx.make_current();
        FrameContext { frame_nr: self.frame_nr }
    }

    fn end_frame(&mut self, _ctx: FrameContext) {}

    fn present(&mut self) {
        let _ = self.ctx.swap_buffers();
    }

    fn resize(&mut self, physical_size: Size) {
        self.viewport = physical_size;
        log::debug!("ltk-gl: resized to {physical_size:?}");
    }

    fn viewport(&self) -> Size { self.viewport }
}
