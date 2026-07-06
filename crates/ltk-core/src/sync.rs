//! Framework-aware sync primitives.

pub use parking_lot::{Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

/// A wrapper that asserts its contents are only accessed on the UI thread.
pub struct UiCell<T>(T);

impl<T> UiCell<T> {
    pub fn new(value: T) -> Self { Self(value) }

    /// Access the inner value. Panics in debug builds if not on UI thread.
    pub fn get(&self) -> &T {
        #[cfg(debug_assertions)]
        assert!(crate::task::is_ui_thread(), "UiCell accessed from non-UI thread");
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut T { &mut self.0 }
}

// SAFETY: UiCell is only accessed after asserting UI thread in debug builds.
unsafe impl<T: Send> Send for UiCell<T> {}
unsafe impl<T: Send> Sync for UiCell<T> {}
