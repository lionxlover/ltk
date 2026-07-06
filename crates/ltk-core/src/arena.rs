//! Simple typed arena allocator for widget/layout node storage.

use slotmap::{DefaultKey, SlotMap};

/// A typed arena backed by a slotmap. O(1) insert, O(1) remove, O(1) lookup.
pub struct Arena<T> {
    inner: SlotMap<DefaultKey, T>,
}

pub type ArenaKey = DefaultKey;

impl<T> Arena<T> {
    pub fn new() -> Self { Self { inner: SlotMap::new() } }
    pub fn with_capacity(cap: usize) -> Self { Self { inner: SlotMap::with_capacity(cap) } }

    pub fn insert(&mut self, value: T) -> ArenaKey { self.inner.insert(value) }
    pub fn remove(&mut self, key: ArenaKey) -> Option<T> { self.inner.remove(key) }
    pub fn get(&self, key: ArenaKey) -> Option<&T> { self.inner.get(key) }
    pub fn get_mut(&mut self, key: ArenaKey) -> Option<&mut T> { self.inner.get_mut(key) }
    pub fn contains(&self, key: ArenaKey) -> bool { self.inner.contains_key(key) }
    pub fn len(&self) -> usize { self.inner.len() }
    pub fn is_empty(&self) -> bool { self.inner.is_empty() }
    pub fn iter(&self) -> impl Iterator<Item = (ArenaKey, &T)> { self.inner.iter() }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (ArenaKey, &mut T)> { self.inner.iter_mut() }
}

impl<T> Default for Arena<T> { fn default() -> Self { Self::new() } }
