//! Paint: fill style for canvas draw commands.

use ltk_core::{color::Color, geometry::Point};

/// A gradient color stop.
#[derive(Debug, Clone)]
pub struct GradientStop { pub offset: f32, pub color: Color }

/// Gradient descriptor.
#[derive(Debug, Clone)]
pub enum Gradient {
    Linear { from: Point, to: Point, stops: Vec<GradientStop> },
    Radial { center: Point, radius: f32, stops: Vec<GradientStop> },
    Conical { center: Point, start_angle: f32, stops: Vec<GradientStop> },
}

/// A paint style for filling or stroking shapes.
#[derive(Debug, Clone)]
pub enum Paint {
    Color(Color),
    Gradient(Gradient),
    Pattern { image_id: u64, repeat_x: bool, repeat_y: bool },
}

impl Paint {
    pub fn solid(c: Color) -> Self { Self::Color(c) }
    pub fn linear_gradient(from: Point, to: Point, stops: Vec<GradientStop>) -> Self {
        Self::Gradient(Gradient::Linear { from, to, stops })
    }
    pub fn is_transparent(&self) -> bool {
        matches!(self, Self::Color(c) if c.alpha() == 0.0)
    }
}

impl From<Color> for Paint {
    fn from(c: Color) -> Self { Self::Color(c) }
}
