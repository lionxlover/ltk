//! Type-erased callable storage for event handlers and property observers.

use std::sync::{Arc, Weak};

/// A heap-allocated type-erased callback that can be called with `&A`.
pub struct Callback<A = ()>(Arc<dyn Fn(&A) + Send + Sync>);

impl<A> Callback<A> {
    pub fn new(f: impl Fn(&A) + Send + Sync + 'static) -> Self {
        Self(Arc::new(f))
    }
    #[inline]
    pub fn call(&self, arg: &A) { (self.0)(arg) }
    pub fn downgrade(&self) -> WeakCallback<A> { WeakCallback(Arc::downgrade(&self.0)) }
}

impl<A> Clone for Callback<A> {
    fn clone(&self) -> Self { Self(Arc::clone(&self.0)) }
}

impl<A> std::fmt::Debug for Callback<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Callback<{}>", std::any::type_name::<A>())
    }
}

/// Weak reference to a [`Callback`]; does not keep the callback alive.
pub struct WeakCallback<A = ()>(Weak<dyn Fn(&A) + Send + Sync>);

impl<A> WeakCallback<A> {
    /// Returns `true` if the callback is still alive.
    pub fn is_alive(&self) -> bool { self.0.strong_count() > 0 }
    /// Attempt to call the callback; returns `false` if it was dropped.
    pub fn try_call(&self, arg: &A) -> bool {
        if let Some(cb) = self.0.upgrade() { cb(arg); true } else { false }
    }
}

/// A list of callbacks that auto-compacts dead weak references.
#[derive(Default)]
pub struct CallbackList<A = ()>(Vec<WeakCallback<A>>);

impl<A> CallbackList<A> {
    pub fn push(&mut self, cb: WeakCallback<A>) { self.0.push(cb) }

    pub fn fire(&mut self, arg: &A) {
        self.0.retain(|cb| cb.try_call(arg));
    }

    pub fn len(&self) -> usize { self.0.len() }
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
}
