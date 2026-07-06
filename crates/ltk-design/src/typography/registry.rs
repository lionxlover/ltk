//! Font registry: loading and looking up font faces.

use std::collections::HashMap;
use std::path::PathBuf;
use ltk_core::LtkResult;

/// Unique ID for a loaded font face.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FontId(u32);

/// Metadata for a single font face (file on disk or memory buffer).
#[derive(Debug, Clone)]
pub struct FontFaceInfo {
    pub id:       FontId,
    pub family:   String,
    pub weight:   u16,
    pub italic:   bool,
    pub path:     Option<PathBuf>,
}

/// Central registry for all font faces known to the application.
pub struct FontRegistry {
    next_id: u32,
    faces:   HashMap<FontId, FontFaceInfo>,
    by_family: HashMap<String, Vec<FontId>>,
}

impl FontRegistry {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            faces:   HashMap::new(),
            by_family: HashMap::new(),
        }
    }

    /// Register a font face from a file path.
    pub fn register_file(&mut self, family: impl Into<String>, path: PathBuf, weight: u16, italic: bool) -> FontId {
        let id = FontId(self.next_id);
        self.next_id += 1;
        let family = family.into();
        let info = FontFaceInfo { id, family: family.clone(), weight, italic, path: Some(path) };
        self.faces.insert(id, info);
        self.by_family.entry(family).or_default().push(id);
        id
    }

    /// Load all system fonts via fontconfig.
    pub fn load_system_fonts(&mut self) -> LtkResult<usize> {
        // In a real implementation: use fontdb to scan /usr/share/fonts etc.
        log::info!("Scanning system fonts…");
        Ok(0) // placeholder
    }

    /// Find the best matching face for a family + weight + italic.
    pub fn best_match(&self, family: &str, weight: u16, italic: bool) -> Option<&FontFaceInfo> {
        let ids = self.by_family.get(family)?;
        ids.iter()
            .filter_map(|id| self.faces.get(id))
            .filter(|f| f.italic == italic)
            .min_by_key(|f| (f.weight as i32 - weight as i32).abs())
    }

    pub fn get(&self, id: FontId) -> Option<&FontFaceInfo> { self.faces.get(&id) }
    pub fn family_names(&self) -> impl Iterator<Item = &str> {
        self.by_family.keys().map(|s| s.as_str())
    }
}

impl Default for FontRegistry { fn default() -> Self { Self::new() } }
