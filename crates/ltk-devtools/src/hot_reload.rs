//! File watcher → diff → hot-patch live UI without restarting the app.

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ReloadEvent { pub path: PathBuf, pub kind: ReloadKind }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReloadKind { SlintFile, RustHotpatch, ThemeFile, AssetFile }

pub struct HotReloadWatcher {
    pub watched_paths: Vec<PathBuf>,
    pub enabled:       bool,
}

impl HotReloadWatcher {
    pub fn new() -> Self {
        let enabled = std::env::var("LTK_HOT_RELOAD").map(|v| v == "1").unwrap_or(cfg!(debug_assertions));
        Self { watched_paths: Vec::new(), enabled }
    }

    pub fn watch(&mut self, path: PathBuf) {
        if self.enabled {
            log::info!("Hot-reload: watching {:?}", path);
            self.watched_paths.push(path);
        }
    }

    /// Poll for changes — call once per frame in debug builds.
    pub fn poll(&self) -> Vec<ReloadEvent> { Vec::new() }
}

impl Default for HotReloadWatcher { fn default() -> Self { Self::new() } }
