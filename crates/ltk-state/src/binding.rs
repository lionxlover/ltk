//! Binding: two-way synchronisation between two signals.

use crate::signal::Signal;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

/// A handle to an active binding. Drop to disconnect.
pub struct BindingHandle { _guard: Arc<()> }

/// Create a two-way binding between two signals of the same type.
/// Changes to either propagate to the other.
pub fn bind<T: Clone + PartialEq + 'static>(a: Signal<T>, b: Signal<T>) -> BindingHandle {
    let guard = Arc::new(());
    let guard_a = Arc::downgrade(&guard);
    let guard_b = Arc::downgrade(&guard);
    let a2 = a.clone();
    let b2 = b.clone();
    let updating = Arc::new(AtomicBool::new(false));
    let upd_a = Arc::clone(&updating);
    let upd_b = Arc::clone(&updating);

    a.subscribe(move |v| {
        if guard_a.upgrade().is_none() { return; }
        if upd_a.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
            if b2.get() != *v { b2.set(v.clone()); }
            upd_a.store(false, Ordering::SeqCst);
        }
    });

    b.subscribe(move |v| {
        if guard_b.upgrade().is_none() { return; }
        if upd_b.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
            if a2.get() != *v { a2.set(v.clone()); }
            upd_b.store(false, Ordering::SeqCst);
        }
    });

    BindingHandle { _guard: guard }
}
