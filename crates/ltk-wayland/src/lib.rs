//! # ltk-wayland — Wayland Protocol Implementations
//!
//! Wraps `wayland-client` + the XDG shell / decoration / DnD protocol
//! extensions behind a stable LTK-facing API. Compiles to a no-op stub
//! when the `live-wayland` feature is disabled (useful for headless CI).

pub mod connection;
pub mod xdg_shell;
pub mod decoration;
pub mod seat;
pub mod output;
pub mod dnd_protocol;

pub use connection::WaylandConnection;
pub use xdg_shell::XdgSurface;
pub use seat::WaylandSeat;
