//! Async task executor interface.

use std::future::Future;
use once_cell::sync::OnceCell;

static UI_THREAD_ID: OnceCell<std::thread::ThreadId> = OnceCell::new();

/// Register the current thread as the UI thread. Call once at startup.
pub fn register_ui_thread() {
    UI_THREAD_ID.set(std::thread::current().id()).ok();
}

/// Returns true if the calling thread is the registered UI thread.
pub fn is_ui_thread() -> bool {
    UI_THREAD_ID.get().map_or(false, |id| *id == std::thread::current().id())
}

/// Abstraction over the runtime executor.
pub trait Executor: Send + Sync + 'static {
    /// Spawn a future on the UI thread (can update widgets).
    fn spawn_ui(&self, fut: impl Future<Output = ()> + 'static);
    /// Spawn a future on a background thread pool.
    fn spawn_bg(&self, fut: impl Future<Output = ()> + Send + 'static);
}
