//! Extension point system: hooks + contributions (VSCode-style extensibility).

use std::collections::HashMap;

/// A named extension point that plugins can contribute to.
pub struct ExtensionPoint {
    pub name:          String,
    pub contributions: Vec<Contribution>,
}

#[derive(Debug, Clone)]
pub struct Contribution {
    pub plugin_id: ltk_core::id::PluginId,
    pub data:      serde_json::Value,
}

pub struct ExtensionManager {
    points: HashMap<String, ExtensionPoint>,
}

impl ExtensionManager {
    pub fn new() -> Self { Self { points: HashMap::new() } }

    pub fn declare_point(&mut self, name: impl Into<String>) {
        let name = name.into();
        self.points.entry(name.clone()).or_insert_with(|| ExtensionPoint { name, contributions: Vec::new() });
    }

    pub fn contribute(&mut self, point: &str, plugin_id: ltk_core::id::PluginId, data: serde_json::Value) {
        if let Some(p) = self.points.get_mut(point) {
            p.contributions.push(Contribution { plugin_id, data });
        }
    }

    pub fn contributions(&self, point: &str) -> &[Contribution] {
        self.points.get(point).map(|p| p.contributions.as_slice()).unwrap_or(&[])
    }
}

impl Default for ExtensionManager { fn default() -> Self { Self::new() } }
