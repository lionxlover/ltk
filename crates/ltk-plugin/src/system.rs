//! Plugin lifecycle: manifest, load/unload, version checks.

use ltk_core::{id::PluginId, version::Version, LtkResult};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// Plugin manifest (typically `plugin.toml`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub name:           String,
    pub version:        String,
    pub ltk_min_version:String,
    pub entry_point:    String,    // .wasm or .so path relative to manifest
    pub permissions:    Vec<String>,
    pub description:    Option<String>,
}

/// A loaded, running plugin instance.
pub struct PluginHandle {
    pub id:       PluginId,
    pub manifest: PluginManifest,
    pub path:     PathBuf,
    pub enabled:  bool,
}

/// Central plugin manager.
pub struct PluginSystem {
    pub plugins:    Vec<PluginHandle>,
    pub ltk_version: Version,
}

impl PluginSystem {
    pub fn new(ltk_version: Version) -> Self { Self { plugins: Vec::new(), ltk_version } }

    pub fn load_manifest(&self, path: &PathBuf) -> LtkResult<PluginManifest> {
        let content = std::fs::read_to_string(path)?;
        toml::from_str(&content)
            .map_err(|e| ltk_core::error::LtkError::Plugin {
                plugin: path.display().to_string(),
                detail: e.to_string(),
            })
    }

    pub fn check_compatibility(&self, manifest: &PluginManifest) -> LtkResult<()> {
        let required = Version::parse(&manifest.ltk_min_version)
            .ok_or_else(|| ltk_core::error::LtkError::Plugin {
                plugin: manifest.name.clone(),
                detail: "invalid version string".into(),
            })?;
        if !self.ltk_version.is_compatible_with(required) {
            return Err(ltk_core::error::LtkError::VersionMismatch {
                required: required.to_string(),
                found:    self.ltk_version.to_string(),
            });
        }
        Ok(())
    }

    pub fn load(&mut self, manifest_path: PathBuf) -> LtkResult<PluginId> {
        let manifest = self.load_manifest(&manifest_path)?;
        self.check_compatibility(&manifest)?;
        let id = PluginId::new();
        self.plugins.push(PluginHandle {
            id, manifest, path: manifest_path, enabled: true,
        });
        log::info!("Plugin loaded: {}", self.plugins.last().unwrap().manifest.name);
        Ok(id)
    }

    pub fn unload(&mut self, id: PluginId) {
        self.plugins.retain(|p| p.id != id);
    }

    pub fn get(&self, id: PluginId) -> Option<&PluginHandle> {
        self.plugins.iter().find(|p| p.id == id)
    }
}
