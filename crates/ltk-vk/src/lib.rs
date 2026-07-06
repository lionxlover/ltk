//! # ltk-vk — Vulkan 1.3 Rendering Backend
//!
//! Implements [`ltk_render::backend_api::RenderBackend`] using Vulkan 1.3
//! via `VK_KHR_surface` + `VK_KHR_wayland_surface`. Preferred backend on
//! systems with a modern GPU driver; falls back to `ltk-gl` otherwise.

pub mod instance;
pub mod device;
pub mod swapchain;
pub mod pipeline;
pub mod backend;

pub use backend::VkBackend;
pub use instance::VkInstance;
