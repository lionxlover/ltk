//! Load themes from TOML / JSON files.

use super::Theme;
use ltk_core::LtkResult;
use std::path::Path;

/// Loads a theme from disk.
pub struct ThemeLoader;

impl ThemeLoader {
    /// Load a theme from a `.toml` file.
    pub fn from_toml(path: &Path) -> LtkResult<Theme> {
        let _content = std::fs::read_to_string(path)?;
        // Full implementation: parse TOML into a ThemeManifest struct,
        // then resolve tokens, build Theme.
        log::info!("Loading theme from {:?}", path);
        Ok(Theme::default_dark()) // placeholder
    }

    /// Load a theme from a `.json` file.
    pub fn from_json(path: &Path) -> LtkResult<Theme> {
        let _content = std::fs::read_to_string(path)?;
        log::info!("Loading theme from {:?}", path);
        Ok(Theme::default_dark()) // placeholder
    }

    /// Load a theme from a directory (looks for `theme.toml` or `theme.json`).
    pub fn from_dir(dir: &Path) -> LtkResult<Theme> {
        let toml_path = dir.join("theme.toml");
        let json_path = dir.join("theme.json");
        if toml_path.exists() { Self::from_toml(&toml_path) }
        else if json_path.exists() { Self::from_json(&json_path) }
        else {
            Err(ltk_core::error::LtkError::ResourceNotFound {
                name: format!("theme.toml/json in {}", dir.display())
            })
        }
    }
}
