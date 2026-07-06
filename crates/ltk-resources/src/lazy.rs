//! Lazy<T>: a value that initialises on first access.

use once_cell::sync::OnceCell;
use std::future::Future;

/// A lazily-initialized value (synchronous).
pub struct Lazy<T, F = fn() -> T> {
    cell: OnceCell<T>,
    init: F,
}

impl<T, F: Fn() -> T> Lazy<T, F> {
    pub const fn new(f: F) -> Self { Self { cell: OnceCell::new(), init: f } }
    pub fn get(&self) -> &T { self.cell.get_or_init(&self.init) }
    pub fn is_initialized(&self) -> bool { self.cell.get().is_some() }
}

impl<T: std::fmt::Debug, F: Fn() -> T> std::fmt::Debug for Lazy<T, F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_initialized() { write!(f, "Lazy({:?})", self.get()) }
        else { write!(f, "Lazy(<uninit>)") }
    }
}

/// Placeholder for an async lazy value (full implementation uses tokio::sync::OnceCell).
pub struct AsyncLazy<T> {
    cell: once_cell::sync::OnceCell<T>,
}

impl<T: Send + Sync> AsyncLazy<T> {
    pub fn new() -> Self { Self { cell: once_cell::sync::OnceCell::new() } }
    pub fn get(&self) -> Option<&T> { self.cell.get() }
    pub fn set(&self, value: T) -> Result<(), T> { self.cell.set(value) }
    pub fn is_ready(&self) -> bool { self.cell.get().is_some() }
}
