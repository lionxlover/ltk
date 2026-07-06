//! # ltk-plugin — Plugin System · WASM Sandbox · Component Registry · Service DI
pub mod system;
pub mod wasm;
pub mod component_registry;
pub mod service_registry;
pub mod extension_manager;

pub use system::{PluginManifest, PluginSystem, PluginHandle};
pub use component_registry::ComponentRegistry;
pub use service_registry::{ServiceRegistry, ServiceId};
pub use extension_manager::{ExtensionPoint, ExtensionManager};
