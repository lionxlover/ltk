//! VkBackend: RenderBackend implementation over Vulkan 1.3.

use ltk_render::backend_api::{RenderBackend, BackendCapabilities, FrameContext};
use ltk_core::geometry::Size;
use crate::{instance::VkInstance, device::VkDevice, swapchain::Swapchain};

pub struct VkBackend {
    instance:   VkInstance,
    device:     VkDevice,
    swapchain:  Swapchain,
    frame_nr:   u64,
}

impl VkBackend {
    pub fn new(app_name: &str, initial_size: Size, vsync: bool) -> ltk_core::LtkResult<Self> {
        let instance  = VkInstance::create(app_name)?;
        let device    = VkDevice::select_best(&instance)?;
        let swapchain = Swapchain::create(initial_size, vsync)?;
        Ok(Self { instance, device, swapchain, frame_nr: 0 })
    }
}

impl RenderBackend for VkBackend {
    fn name(&self) -> &str { "Vulkan 1.3" }

    fn capabilities(&self) -> BackendCapabilities {
        BackendCapabilities {
            name: format!("Vulkan 1.3 ({})", self.device.gpu.name),
            max_texture_size: 16384,
            supports_msaa: true,
            supports_hdr: true,
            supports_blend_modes: true,
        }
    }

    fn begin_frame(&mut self) -> FrameContext {
        self.frame_nr += 1;
        FrameContext { frame_nr: self.frame_nr }
    }

    fn end_frame(&mut self, _ctx: FrameContext) {}
    fn present(&mut self) {}

    fn resize(&mut self, physical_size: Size) {
        self.swapchain.recreate(physical_size);
    }

    fn viewport(&self) -> Size { self.swapchain.size }
}
