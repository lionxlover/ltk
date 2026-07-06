//! Physical/logical device selection and queue setup.

use ltk_core::LtkResult;

#[derive(Debug, Clone)]
pub struct GpuInfo {
    pub name:        String,
    pub vendor_id:   u32,
    pub device_type: GpuType,
    pub vram_mb:     u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuType { DiscreteGpu, IntegratedGpu, VirtualGpu, Cpu, Other }

pub struct VkDevice { pub gpu: GpuInfo }

impl VkDevice {
    /// Select the best available physical device (prefer discrete GPU).
    pub fn select_best(_instance: &crate::instance::VkInstance) -> LtkResult<Self> {
        log::info!("ltk-vk: enumerating physical devices");
        // Real impl: vkEnumeratePhysicalDevices, score by device_type + VRAM
        Ok(Self {
            gpu: GpuInfo {
                name: "Unknown GPU".into(),
                vendor_id: 0,
                device_type: GpuType::DiscreteGpu,
                vram_mb: 0,
            },
        })
    }
}
