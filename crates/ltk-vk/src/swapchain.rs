//! Swapchain management for presenting frames to a Wayland surface.

use ltk_core::geometry::Size;
use ltk_core::LtkResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresentMode { Immediate, Mailbox, Fifo, FifoRelaxed }

pub struct Swapchain {
    pub size:         Size,
    pub image_count:  u32,
    pub present_mode:  PresentMode,
}

impl Swapchain {
    pub fn create(size: Size, vsync: bool) -> LtkResult<Self> {
        let mode = if vsync { PresentMode::Fifo } else { PresentMode::Mailbox };
        log::info!("ltk-vk: creating swapchain {size:?} mode={mode:?}");
        Ok(Self { size, image_count: 3, present_mode: mode })
    }

    pub fn recreate(&mut self, new_size: Size) {
        self.size = new_size;
        log::debug!("ltk-vk: swapchain recreated at {new_size:?}");
    }
}
