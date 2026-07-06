//! Visual snapshot comparison with diff output.

use std::path::{Path, PathBuf};
use ltk_core::LtkResult;

pub struct SnapshotResult { pub matched: bool, pub diff_pixels: u32, pub diff_path: Option<PathBuf> }

pub struct SnapshotTester { pub baseline_dir: PathBuf, pub threshold: f32 }

impl SnapshotTester {
    pub fn new(baseline_dir: PathBuf) -> Self { Self { baseline_dir, threshold: 0.01 } }

    /// Compare `actual` RGBA8 pixels against the stored baseline for `name`.
    pub fn compare(&self, name: &str, actual: &[u8], width: u32, height: u32) -> LtkResult<SnapshotResult> {
        let baseline_path = self.baseline_dir.join(format!("{name}.png"));
        if !baseline_path.exists() {
            // First run: save as new baseline
            self.save_baseline(&baseline_path, actual, width, height)?;
            return Ok(SnapshotResult { matched: true, diff_pixels: 0, diff_path: None });
        }
        let baseline = image::open(&baseline_path)
            .map_err(|e| ltk_core::error::LtkError::internal(e.to_string()))?
            .into_rgba8();

        if baseline.width() != width || baseline.height() != height {
            return Ok(SnapshotResult { matched: false, diff_pixels: u32::MAX, diff_path: None });
        }

        let mut diff_count = 0u32;
        for (a, b) in actual.chunks(4).zip(baseline.pixels()) {
            if a != b.0.as_slice() { diff_count += 1; }
        }
        let total_pixels = width * height;
        let diff_ratio = diff_count as f32 / total_pixels as f32;

        Ok(SnapshotResult { matched: diff_ratio <= self.threshold, diff_pixels: diff_count, diff_path: None })
    }

    fn save_baseline(&self, path: &Path, pixels: &[u8], w: u32, h: u32) -> LtkResult<()> {
        std::fs::create_dir_all(self.baseline_dir.as_path())?;
        image::save_buffer(path, pixels, w, h, image::ColorType::Rgba8)
            .map_err(|e| ltk_core::error::LtkError::internal(e.to_string()))?;
        Ok(())
    }
}
