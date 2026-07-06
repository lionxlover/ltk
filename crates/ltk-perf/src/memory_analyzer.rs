//! Memory usage analysis for arenas, caches, and texture budgets.

#[derive(Debug, Clone, Default)]
pub struct MemoryReport {
    pub arena_bytes:      usize,
    pub texture_bytes:    usize,
    pub image_cache_bytes:usize,
    pub font_cache_bytes: usize,
    pub total_bytes:      usize,
    pub peak_bytes:       usize,
}

pub struct MemoryAnalyzer { pub report: MemoryReport }

impl MemoryAnalyzer {
    pub fn new() -> Self { Self { report: MemoryReport::default() } }

    pub fn update(&mut self, arenas: usize, textures: usize, images: usize, fonts: usize) {
        self.report.arena_bytes       = arenas;
        self.report.texture_bytes     = textures;
        self.report.image_cache_bytes = images;
        self.report.font_cache_bytes  = fonts;
        self.report.total_bytes       = arenas + textures + images + fonts;
        self.report.peak_bytes        = self.report.peak_bytes.max(self.report.total_bytes);
    }

    pub fn total_mb(&self) -> f64 { self.report.total_bytes as f64 / (1024.0 * 1024.0) }
    pub fn peak_mb(&self)  -> f64 { self.report.peak_bytes  as f64 / (1024.0 * 1024.0) }
}

impl Default for MemoryAnalyzer { fn default() -> Self { Self::new() } }
