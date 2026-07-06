//! Generic LRU asset cache with byte-budget eviction.

use ltk_core::id::ResourceId;
use std::collections::{HashMap, VecDeque};

struct CacheEntry { data: Vec<u8>, cost: usize, last_used: u64 }

/// A size-bounded LRU cache for decoded/processed assets.
pub struct AssetCache {
    entries:    HashMap<ResourceId, CacheEntry>,
    order:      VecDeque<ResourceId>,   // LRU order
    total_cost: usize,
    max_cost:   usize,
    frame_nr:   u64,
}

impl AssetCache {
    pub fn new(max_bytes: usize) -> Self {
        Self { entries: HashMap::new(), order: VecDeque::new(),
               total_cost: 0, max_cost: max_bytes, frame_nr: 0 }
    }

    pub fn insert(&mut self, id: ResourceId, data: Vec<u8>) {
        let cost = data.len();
        self.evict_to_fit(cost);
        self.total_cost += cost;
        self.entries.insert(id, CacheEntry { data, cost, last_used: self.frame_nr });
        self.order.retain(|&x| x != id);
        self.order.push_back(id);
    }

    pub fn get(&mut self, id: ResourceId) -> Option<&[u8]> {
        if let Some(e) = self.entries.get_mut(&id) {
            e.last_used = self.frame_nr;
            self.order.retain(|&x| x != id);
            self.order.push_back(id);
            Some(&e.data)
        } else { None }
    }

    fn evict_to_fit(&mut self, needed: usize) {
        while self.total_cost + needed > self.max_cost {
            if let Some(oldest) = self.order.pop_front() {
                if let Some(e) = self.entries.remove(&oldest) {
                    self.total_cost -= e.cost;
                }
            } else { break; }
        }
    }

    pub fn advance_frame(&mut self) { self.frame_nr += 1; }
    pub fn total_bytes(&self) -> usize { self.total_cost }
    pub fn count(&self) -> usize { self.entries.len() }
    pub fn hit_rate(&self) -> f32 { 0.0 } // implement with counters in prod
}
