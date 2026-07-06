//! Register custom Slint/Rust components at runtime (plugin-contributed widgets).

use std::collections::HashMap;
use std::sync::Arc;

/// A factory function that creates a component instance (type-erased).
pub type ComponentFactory = Arc<dyn Fn() -> Box<dyn std::any::Any + Send> + Send + Sync>;

pub struct ComponentRegistry {
    factories: HashMap<String, ComponentFactory>,
}

impl ComponentRegistry {
    pub fn new() -> Self { Self { factories: HashMap::new() } }

    pub fn register(&mut self, name: impl Into<String>, factory: ComponentFactory) {
        self.factories.insert(name.into(), factory);
    }

    pub fn create(&self, name: &str) -> Option<Box<dyn std::any::Any + Send>> {
        self.factories.get(name).map(|f| f())
    }

    pub fn is_registered(&self, name: &str) -> bool { self.factories.contains_key(name) }
    pub fn names(&self) -> impl Iterator<Item = &str> { self.factories.keys().map(|s| s.as_str()) }
}

impl Default for ComponentRegistry { fn default() -> Self { Self::new() } }
