//! Vulkan instance creation with validation layers in debug builds.

use ltk_core::LtkResult;

pub struct VkInstance { pub api_version: (u32, u32, u32), pub validation_enabled: bool }

impl VkInstance {
    pub fn create(app_name: &str) -> LtkResult<Self> {
        let validation = cfg!(debug_assertions);
        log::info!("ltk-vk: creating Vulkan instance for '{app_name}' (validation={validation})");
        // Real impl: ash::Entry::load() + InstanceCreateInfo with VK_KHR_wayland_surface
        Ok(Self { api_version: (1, 3, 0), validation_enabled: validation })
    }
}
