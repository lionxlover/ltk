//! Display server connection (Wayland / X11 abstraction).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayBackend { Wayland, X11, Offscreen }

pub struct DisplayConnection {
    pub backend:  DisplayBackend,
    pub connected:bool,
}

impl DisplayConnection {
    pub fn connect() -> ltk_core::LtkResult<Self> {
        let backend = if std::env::var("WAYLAND_DISPLAY").is_ok() {
            DisplayBackend::Wayland
        } else if std::env::var("DISPLAY").is_ok() {
            DisplayBackend::X11
        } else {
            DisplayBackend::Offscreen
        };
        log::info!("Display backend: {:?}", backend);
        Ok(Self { backend, connected: true })
    }
    pub fn is_wayland(&self) -> bool { self.backend == DisplayBackend::Wayland }
    pub fn is_x11(&self)     -> bool { self.backend == DisplayBackend::X11 }
}
