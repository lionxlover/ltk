//! Central resource registry with named resources and reference counting.

use ltk_core::{id::ResourceId, string::SharedString, LtkResult, error::LtkError};
use std::{collections::HashMap, path::PathBuf, sync::Arc};

/// Category of resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceKind { Image, Font, Theme, Icon, Audio, Other }

/// A handle to a loaded resource.
#[derive(Debug, Clone)]
pub struct ResourceHandle {
    pub id:   ResourceId,
    pub name: SharedString,
    pub kind: ResourceKind,
    pub path: Option<PathBuf>,
    ref_count: Arc<()>,   // dropped when last handle is dropped
}

impl ResourceHandle {
    pub fn ref_count(&self) -> usize { Arc::strong_count(&self.ref_count) }
}

struct ResourceEntry {
    handle:    ResourceHandle,
    data:      Vec<u8>,
}

/// Named resource registry.
pub struct ResourceManager {
    entries:   HashMap<ResourceId, ResourceEntry>,
    by_name:   HashMap<SharedString, ResourceId>,
}

impl ResourceManager {
    pub fn new() -> Self { Self { entries: HashMap::new(), by_name: HashMap::new() } }

    /// Register raw bytes under a name.
    pub fn register(
        &mut self,
        name: impl Into<String>,
        kind: ResourceKind,
        data: Vec<u8>,
        path: Option<PathBuf>,
    ) -> ResourceHandle {
        let id     = ResourceId::new();
        let name   = SharedString::new(name);
        let handle = ResourceHandle { id, name: name.clone(), kind, path, ref_count: Arc::new(()) };
        self.entries.insert(id, ResourceEntry { handle: handle.clone(), data });
        self.by_name.insert(name, id);
        handle
    }

    /// Load a file from disk and register it.
    pub fn load_file(&mut self, path: PathBuf, kind: ResourceKind) -> LtkResult<ResourceHandle> {
        let data = std::fs::read(&path)?;
        let name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unnamed")
            .to_string();
        Ok(self.register(name, kind, data, Some(path)))
    }

    pub fn get_data(&self, id: ResourceId) -> Option<&[u8]> {
        self.entries.get(&id).map(|e| e.data.as_slice())
    }

    pub fn by_name(&self, name: &str) -> Option<ResourceHandle> {
        let id = self.by_name.get(&SharedString::new(name))?;
        self.entries.get(id).map(|e| e.handle.clone())
    }

    /// Remove resources that have no live handles.
    pub fn collect_garbage(&mut self) {
        let stale: Vec<_> = self.entries.iter()
            .filter(|(_, e)| e.handle.ref_count() == 1)
            .map(|(&id, _)| id)
            .collect();
        for id in &stale {
            if let Some(e) = self.entries.remove(id) {
                self.by_name.remove(&e.handle.name);
            }
        }
        log::debug!("ResourceManager: evicted {} stale entries", stale.len());
    }

    pub fn count(&self) -> usize { self.entries.len() }
}

impl Default for ResourceManager { fn default() -> Self { Self::new() } }
