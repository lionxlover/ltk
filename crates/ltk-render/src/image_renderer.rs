//! Image decode and upload pipeline.

use ltk_core::geometry::Size;
use ltk_core::LtkResult;
use std::path::Path;

/// Decoded image ready for GPU upload.
pub struct DecodedImage {
    pub width:  u32,
    pub height: u32,
    pub pixels: Vec<u8>,  // RGBA8
}

impl DecodedImage {
    pub fn size(&self) -> Size { Size::new(self.width as f32, self.height as f32) }
}

/// Loads and decodes images from disk or memory.
pub struct ImageRenderer;

impl ImageRenderer {
    pub fn load_file(path: &Path) -> LtkResult<DecodedImage> {
        let img = image::open(path)
            .map_err(|e| ltk_core::error::LtkError::internal(e.to_string()))?
            .into_rgba8();
        let (w, h) = img.dimensions();
        Ok(DecodedImage { width: w, height: h, pixels: img.into_raw() })
    }

    pub fn load_bytes(data: &[u8]) -> LtkResult<DecodedImage> {
        let img = image::load_from_memory(data)
            .map_err(|e| ltk_core::error::LtkError::internal(e.to_string()))?
            .into_rgba8();
        let (w, h) = img.dimensions();
        Ok(DecodedImage { width: w, height: h, pixels: img.into_raw() })
    }
}
