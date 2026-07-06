//! In-app debug overlay: FPS counter, layout bounds, repaint flash.

#[derive(Debug, Clone, Default)]
pub struct DebugOverlayConfig {
    pub show_fps:          bool,
    pub show_layout_bounds:bool,
    pub show_repaint_flash:bool,
    pub show_memory:       bool,
    pub show_widget_count: bool,
}

pub struct DebugOverlay {
    pub config: DebugOverlayConfig,
    pub fps:    f32,
}

impl DebugOverlay {
    pub fn new() -> Self { Self { config: DebugOverlayConfig::default(), fps: 0.0 } }
    pub fn toggle_fps(&mut self) { self.config.show_fps = !self.config.show_fps; }
    pub fn update_fps(&mut self, fps: f32) { self.fps = fps; }
}

impl Default for DebugOverlay { fn default() -> Self { Self::new() } }
