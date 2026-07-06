//! Detect OS, desktop environment, and capability flags.

use ltk_core::env::{DisplayServer, GpuApi};

#[derive(Debug, Clone)]
pub struct PlatformInfo {
    pub os_name:         String,
    pub os_version:      String,
    pub desktop_env:     Option<String>,
    pub display_server:  DisplayServer,
    pub is_lionos:       bool,
    pub has_dbus:        bool,
    pub has_systemd:     bool,
    pub wayland_version: Option<(u32, u32)>,
}

pub fn detect() -> PlatformInfo {
    let os_name = std::fs::read_to_string("/etc/os-release")
        .ok()
        .and_then(|s| s.lines()
            .find(|l| l.starts_with("NAME="))
            .map(|l| l.trim_start_matches("NAME=").trim_matches('"').to_string()))
        .unwrap_or_else(|| "Linux".into());

    let is_lionos = os_name.to_lowercase().contains("lionos");
    let desktop_env = std::env::var("XDG_CURRENT_DESKTOP").ok();

    let display_server = if std::env::var("WAYLAND_DISPLAY").is_ok() {
        DisplayServer::Wayland
    } else if std::env::var("DISPLAY").is_ok() {
        DisplayServer::X11
    } else {
        DisplayServer::OffScreen
    };

    let has_dbus = std::env::var("DBUS_SESSION_BUS_ADDRESS").is_ok();

    PlatformInfo {
        os_name,
        os_version: "unknown".into(),
        desktop_env,
        display_server,
        is_lionos,
        has_dbus,
        has_systemd: std::path::Path::new("/run/systemd").exists(),
        wayland_version: None,
    }
}
