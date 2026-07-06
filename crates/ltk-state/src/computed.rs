//! Computed<T>: a lazily-evaluated, memoized derived value.

use crate::signal::Signal;
use ltk_core::sync::RwLock;
use std::sync::Arc;

struct ComputedInner<T> {
    value:  Option<T>,
    dirty:  bool,
    compute: Box<dyn Fn() -> T + Send + Sync>,
}

/// A memoized derived value. Recomputes only when marked dirty.
pub struct Computed<T: Clone + 'static>(Arc<RwLock<ComputedInner<T>>>);

impl<T: Clone + 'static> Computed<T> {
    /// Create a new computed value from a closure.
    pub fn new(f: impl Fn() -> T + Send + Sync + 'static) -> Self {
        Self(Arc::new(RwLock::new(ComputedInner {
            value:   None,
            dirty:   true,
            compute: Box::new(f),
        })))
    }

    /// Get the current value, recomputing if dirty.
    pub fn get(&self) -> T {
        let mut inner = self.0.write();
        if inner.dirty {
            inner.value = Some((inner.compute)());
            inner.dirty = false;
        }
        inner.value.clone().unwrap()
    }

    /// Mark this computed as needing recomputation on next `.get()`.
    pub fn invalidate(&self) { self.0.write().dirty = true; }
}

impl<T: Clone + 'static> Clone for Computed<T> {
    fn clone(&self) -> Self { Self(Arc::clone(&self.0)) }
}

/// Helper function to create a computed value.
pub fn computed<T: Clone + 'static>(f: impl Fn() -> T + Send + Sync + 'static) -> Computed<T> {
    Computed::new(f)
}
