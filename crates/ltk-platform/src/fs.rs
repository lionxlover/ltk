//! Filesystem watcher and XDG directory helpers.

use std::path::{Path, PathBuf};
use ltk_core::LtkResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WatchEvent {
    Created(PathBuf),
    Modified(PathBuf),
    Deleted(PathBuf),
    Renamed { from: PathBuf, to: PathBuf },
}

/// Filesystem watcher (wraps inotify on Linux).
pub struct FsWatcher {
    pub watched: Vec<PathBuf>,
}

impl FsWatcher {
    pub fn new() -> Self { Self { watched: Vec::new() } }

    pub fn watch(&mut self, path: impl AsRef<Path>) -> LtkResult<()> {
        self.watched.push(path.as_ref().to_path_buf());
        log::debug!("FsWatcher: watching {:?}", path.as_ref());
        Ok(())
    }

    pub fn unwatch(&mut self, path: impl AsRef<Path>) {
        self.watched.retain(|p| p != path.as_ref());
    }

    /// Poll for pending events (non-blocking). Returns empty Vec if none.
    pub fn poll_events(&self) -> Vec<WatchEvent> { Vec::new() }
}

/// XDG base directory helpers.
pub struct XdgDirs;

impl XdgDirs {
    pub fn config_home() -> PathBuf {
        std::env::var("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                let mut p = dirs_home(); p.push(".config"); p
            })
    }

    pub fn data_home() -> PathBuf {
        std::env::var("XDG_DATA_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                let mut p = dirs_home(); p.push(".local/share"); p
            })
    }

    pub fn cache_home() -> PathBuf {
        std::env::var("XDG_CACHE_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                let mut p = dirs_home(); p.push(".cache"); p
            })
    }

    pub fn runtime_dir() -> Option<PathBuf> {
        std::env::var("XDG_RUNTIME_DIR").ok().map(PathBuf::from)
    }
}

fn dirs_home() -> PathBuf {
    std::env::var("HOME").map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp"))
}

impl Default for FsWatcher { fn default() -> Self { Self::new() } }
