//! Runtime environment detection: display server, GPU, accessibility, etc.

use once_cell::sync::Lazy;

/// Detected runtime environment capabilities.
#[derive(Debug, Clone)]
pub struct Environment {
    pub display_server: DisplayServer,
    pub gpu_api:        GpuApi,
    pub a11y_enabled:   bool,
    pub hdr_support:    bool,
    pub touch_screen:   bool,
    pub high_dpi:       bool,
    pub scale_factor:   f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DisplayServer { Wayland, X11, OffScreen }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GpuApi { Vulkan, OpenGL, Software }

impl Environment {
    /// Detect the runtime environment. Called once at startup.
    pub fn detect() -> Self {
        let display_server = if std::env::var("WAYLAND_DISPLAY").is_ok() {
            DisplayServer::Wayland
        } else if std::env::var("DISPLAY").is_ok() {
            DisplayServer::X11
        } else {
            DisplayServer::OffScreen
        };

        let a11y_enabled = std::env::var("GNOME_ACCESSIBILITY")
            .map(|v| v == "1")
            .unwrap_or(false);

        Self {
            display_server,
            gpu_api: GpuApi::Vulkan, // detected properly at render-init
            a11y_enabled,
            hdr_support: false,
            touch_screen: false,
            high_dpi: false,
            scale_factor: 1.0,
        }
    }
}

/// Global environment, lazily initialized on first access.
pub static ENV: Lazy<Environment> = Lazy::new(Environment::detect);
