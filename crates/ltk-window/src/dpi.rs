//! DPI detection and logical↔physical pixel conversion.

#[derive(Debug, Clone)]
pub struct DpiManager {
    pub scale_factor: f32,       // e.g. 1.0, 1.5, 2.0
    pub ui_scale:     f32,       // user font-size override
    pub physical_dpi: f32,
}

impl DpiManager {
    pub fn new(scale: f32) -> Self {
        Self { scale_factor: scale, ui_scale: 1.0, physical_dpi: 96.0 * scale }
    }

    pub fn from_env() -> Self {
        let scale = std::env::var("GDK_SCALE")
            .ok()
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(1.0);
        Self::new(scale)
    }

    #[inline] pub fn to_physical(&self, logical: f32)  -> f32 { logical  * self.scale_factor * self.ui_scale }
    #[inline] pub fn to_logical(&self,   physical: f32) -> f32 { physical / (self.scale_factor * self.ui_scale) }
    #[inline] pub fn physical_size(&self, s: ltk_core::geometry::Size) -> (u32, u32) {
        ((s.width  * self.scale_factor).ceil() as u32,
         (s.height * self.scale_factor).ceil() as u32)
    }

    pub fn set_user_scale(&mut self, scale: f32) {
        self.ui_scale = scale.clamp(0.75, 3.0);
    }
}

impl Default for DpiManager { fn default() -> Self { Self::new(1.0) } }
