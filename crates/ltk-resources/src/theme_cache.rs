//! Compiled theme token tree cache.

use ltk_core::string::SharedString;
use std::collections::HashMap;

pub struct ThemeCache { compiled: HashMap<SharedString, Vec<u8>> }

impl ThemeCache {
    pub fn new() -> Self { Self { compiled: HashMap::new() } }
    pub fn set(&mut self, name: impl Into<String>, data: Vec<u8>) {
        self.compiled.insert(SharedString::new(name), data);
    }
    pub fn get(&self, name: &str) -> Option<&[u8]> {
        self.compiled.get(&SharedString::new(name)).map(|v| v.as_slice())
    }
    pub fn invalidate(&mut self, name: &str) { self.compiled.remove(&SharedString::new(name)); }
    pub fn invalidate_all(&mut self) { self.compiled.clear(); }
}

impl Default for ThemeCache { fn default() -> Self { Self::new() } }
