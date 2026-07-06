//! # ltk-platform — Linux/LionOS Glue · D-Bus · XDG Portals · FS Watcher
pub mod detection;
pub mod fs;
pub mod dbus;
pub mod portal;

pub use detection::{PlatformInfo, detect};
pub use fs::{FsWatcher, WatchEvent};
pub use portal::XdgPortal;
