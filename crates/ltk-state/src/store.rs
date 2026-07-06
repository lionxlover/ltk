//! Centralized state store with typed selectors.

use crate::signal::Signal;
use std::collections::HashMap;
use std::any::Any;

/// A simple key-value state store backed by type-erased signals.
pub struct Store {
    entries: HashMap<String, Box<dyn Any + Send + Sync>>,
}

impl Store {
    pub fn new() -> Self { Self { entries: HashMap::new() } }

    /// Register a signal under a string key.
    pub fn register<T: Clone + 'static>(&mut self, key: impl Into<String>, initial: T) -> Signal<T> {
        let sig = Signal::new(initial);
        self.entries.insert(key.into(), Box::new(sig.clone()));
        sig
    }

    /// Retrieve a signal by key.
    pub fn select<T: Clone + 'static>(&self, key: &str) -> Option<Signal<T>> {
        self.entries.get(key)
            .and_then(|e| e.downcast_ref::<Signal<T>>())
            .cloned()
    }
}

impl Default for Store { fn default() -> Self { Self::new() } }
