//! OpenGL context creation via EGL (Wayland-compatible).

use ltk_core::LtkResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlProfile { Core46, Es32 }

pub struct GlContext {
    pub profile: GlProfile,
    pub vsync:   bool,
}

impl GlContext {
    /// Create a context, preferring desktop GL 4.6 and falling back to ES 3.2.
    pub fn create(vsync: bool) -> LtkResult<Self> {
        log::info!("ltk-gl: creating OpenGL context (vsync={vsync})");
        // Real impl: glutin::ConfigTemplateBuilder + DisplayBuilder over EGL
        Ok(Self { profile: GlProfile::Core46, vsync })
    }

    pub fn make_current(&self) -> LtkResult<()> { Ok(()) }
    pub fn swap_buffers(&self) -> LtkResult<()> { Ok(()) }
}
