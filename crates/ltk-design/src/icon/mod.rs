//! Icon registry and named icon constants.

use std::collections::HashMap;
use std::path::PathBuf;

/// A named icon identifier.
pub type IconName = &'static str;

/// An icon theme definition (like hicolor or Adwaita).
#[derive(Debug, Clone)]
pub struct IconTheme {
    pub name:    String,
    pub dirs:    Vec<PathBuf>,
    pub fallback: Option<String>,
}

/// Registry mapping icon names to SVG source data or file paths.
pub struct IconRegistry {
    themes:     Vec<IconTheme>,
    cache:      HashMap<(String, u32), IconEntry>, // (name+size) → data
}

#[derive(Debug, Clone)]
pub enum IconEntry {
    SvgData(Vec<u8>),
    FilePath(PathBuf),
}

impl IconRegistry {
    pub fn new() -> Self { Self { themes: Vec::new(), cache: HashMap::new() } }

    pub fn register_theme(&mut self, theme: IconTheme) {
        self.themes.push(theme);
    }

    /// Look up a named icon, returning SVG bytes or a path.
    pub fn lookup(&self, name: &str, size: u32) -> Option<&IconEntry> {
        self.cache.get(&(name.to_string(), size))
    }

    /// Inline-register an SVG icon by name (useful for bundled icons).
    pub fn register_svg(&mut self, name: impl Into<String>, svg: Vec<u8>) {
        // Register at all standard sizes
        for size in [16u32, 20, 24, 32, 48] {
            self.cache.insert((name.to_string(), size), IconEntry::SvgData(svg.clone()));
        }
    }
}

impl Default for IconRegistry { fn default() -> Self { Self::new() } }
