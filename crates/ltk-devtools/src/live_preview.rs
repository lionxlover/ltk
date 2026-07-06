//! Standalone live-preview runner for `.slint` files (like `slint-viewer`).

use std::path::PathBuf;
use ltk_core::LtkResult;

pub struct LivePreview { pub file: PathBuf }

impl LivePreview {
    pub fn open(file: PathBuf) -> LtkResult<Self> {
        if !file.exists() {
            return Err(ltk_core::error::LtkError::ResourceNotFound {
                name: file.display().to_string()
            });
        }
        log::info!("Live preview: opening {:?}", file);
        Ok(Self { file })
    }

    /// Reload the previewed file after an edit.
    pub fn reload(&self) -> LtkResult<()> {
        log::info!("Live preview: reloading {:?}", self.file);
        Ok(())
    }
}
