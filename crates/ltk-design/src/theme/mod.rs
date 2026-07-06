//! Theme engine: token tree, dark/light, hot-swap, serialization.

pub mod engine;
pub mod loader;
pub mod serializer;

pub use engine::{ThemeEngine, ThemeChangeEvent};
pub use loader::ThemeLoader;
pub use serializer::ThemeSerializer;

use crate::color::{ColorPalette, ThemeColors};
use crate::typography::TypeScale;
use crate::spacing::{SpaceScale, LayoutTokens};
use crate::elevation::{ShadowSpec, GlassSpec, ElevationLevel};
use crate::motion::MotionTokens;
use serde::{Deserialize, Serialize};

/// The active display mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ThemeMode { #[default] Dark, Light, System }

/// A fully-resolved LTK theme: all tokens in one struct.
pub struct Theme {
    pub name:    String,
    pub mode:    ThemeMode,
    pub colors:  ColorPalette,   // resolved for current mode
    pub types:   TypeScale,
    pub spaces:  SpaceScale,
    pub layout:  LayoutTokens,
    pub motion:  MotionTokens,
    // Glass presets
    pub glass:   GlassSpec,
    // Radius tokens
    pub radius:  RadiusTokens,
    // Source (for hot-swap)
    pub source:  ThemeColors,
}

/// Border-radius token set.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiusTokens {
    pub xs:     f32,   // 3
    pub sm:     f32,   // 6
    pub md:     f32,   // 10
    pub lg:     f32,   // 14
    pub xl:     f32,   // 18
    pub xl2:    f32,   // 24
    pub xl3:    f32,   // 32
    pub full:   f32,   // 9999
    pub button: f32,   // full (always pill)
    pub card:   f32,   // lg
    pub dialog: f32,   // xl
    pub input:  f32,   // sm
    pub tag:    f32,   // full
}

impl Default for RadiusTokens {
    fn default() -> Self {
        Self {
            xs: 3.0, sm: 6.0, md: 10.0, lg: 14.0, xl: 18.0,
            xl2: 24.0, xl3: 32.0, full: 9999.0,
            button: 9999.0, card: 14.0, dialog: 18.0, input: 8.0, tag: 9999.0,
        }
    }
}

impl Theme {
    /// Build the default LionOS dark theme at 213° primary hue.
    pub fn default_dark() -> Self {
        let source = ThemeColors::from_hue(213.0);
        let mode   = ThemeMode::Dark;
        Self {
            name:   "LionOS Dark".into(),
            mode,
            colors: source.dark.clone(),
            types:  TypeScale::new(14.0),
            spaces: SpaceScale::new(8.0),
            layout: LayoutTokens::default(),
            motion: MotionTokens { reduce_motion: false },
            glass:  GlassSpec::dark(),
            radius: RadiusTokens::default(),
            source,
        }
    }

    /// Build the default LionOS light theme.
    pub fn default_light() -> Self {
        let source = ThemeColors::from_hue(213.0);
        let mode   = ThemeMode::Light;
        Self {
            name:   "LionOS Light".into(),
            mode,
            colors: source.light.clone(),
            types:  TypeScale::new(14.0),
            spaces: SpaceScale::new(8.0),
            layout: LayoutTokens::default(),
            motion: MotionTokens { reduce_motion: false },
            glass:  GlassSpec::light(),
            radius: RadiusTokens::default(),
            source,
        }
    }

    /// Create a copy of this theme with a different accent hue.
    pub fn with_hue(&self, hue: f32) -> Self {
        let source = ThemeColors::from_hue(hue);
        let colors = match self.mode {
            ThemeMode::Dark | ThemeMode::System => source.dark.clone(),
            ThemeMode::Light => source.light.clone(),
        };
        Self {
            name: format!("{} (custom hue {hue:.0}°)", self.name),
            colors,
            source,
            ..self.clone_metadata()
        }
    }

    fn clone_metadata(&self) -> Self {
        Self {
            name:   self.name.clone(),
            mode:   self.mode,
            colors: self.colors.clone(),
            types:  TypeScale::new(self.types.base),
            spaces: SpaceScale::new(self.spaces.base),
            layout: self.layout.clone(),
            motion: MotionTokens { reduce_motion: self.motion.reduce_motion },
            glass:  self.glass.clone(),
            radius: self.radius.clone(),
            source: self.source.clone(),
        }
    }
}
