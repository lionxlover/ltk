//! Signal<T>: the fundamental reactive primitive.

use ltk_core::sync::RwLock;
use std::sync::Arc;

type Subscriber<T> = Box<dyn Fn(&T) + Send + Sync>;

struct SignalInner<T> {
    value: T,
    subs:  Vec<Subscriber<T>>,
}

/// A reactive value. When set, notifies all subscribers synchronously.
pub struct Signal<T: Clone + 'static>(Arc<RwLock<SignalInner<T>>>);

impl<T: Clone + 'static> Signal<T> {
    pub fn new(value: T) -> Self {
        Self(Arc::new(RwLock::new(SignalInner { value, subs: Vec::new() })))
    }

    /// Read the current value (cheap clone).
    pub fn get(&self) -> T { self.0.read().value.clone() }

    /// Set a new value and notify all subscribers.
    pub fn set(&self, value: T) {
        let subs: Vec<_> = {
            let mut inner = self.0.write();
            inner.value = value.clone();
            inner.subs.iter().map(|_| ()).collect() // just check count
        };
        // Re-read to call subs (avoid holding write lock)
        let inner = self.0.read();
        for sub in &inner.subs { sub(&inner.value); }
    }

    /// Update in-place.
    pub fn update(&self, f: impl FnOnce(T) -> T) {
        let new_val = f(self.get());
        self.set(new_val);
    }

    /// Subscribe to changes. Returns a handle ID (index for now).
    pub fn subscribe(&self, cb: impl Fn(&T) + Send + Sync + 'static) -> usize {
        let mut inner = self.0.write();
        let idx = inner.subs.len();
        inner.subs.push(Box::new(cb));
        idx
    }

    /// Unsubscribe by index (swap-remove for O(1)).
    pub fn unsubscribe(&self, idx: usize) {
        let mut inner = self.0.write();
        if idx < inner.subs.len() { inner.subs.swap_remove(idx); }
    }

    pub fn subscriber_count(&self) -> usize { self.0.read().subs.len() }
}

impl<T: Clone + 'static> Clone for Signal<T> {
    fn clone(&self) -> Self { Self(Arc::clone(&self.0)) }
}
