//! 2D drawing command recorder (retained-mode canvas).

use ltk_core::geometry::{Rect, Point};
use crate::paint::Paint;

/// A single recorded draw call.
#[derive(Debug, Clone)]
pub enum DrawCommand {
    PushClip    { rect: Rect, radius: f32 },
    PopClip,
    FillRect    { rect: Rect, paint: Paint },
    StrokeRect  { rect: Rect, paint: Paint, width: f32 },
    FillRoundedRect { rect: Rect, radius: f32, paint: Paint },
    FillCircle  { center: Point, radius: f32, paint: Paint },
    DrawLine    { from: Point, to: Point, paint: Paint, width: f32 },
    DrawText    { x: f32, y: f32, text: String, paint: Paint },
    DrawImage   { src_id: u64, dst: Rect, src: Option<Rect>, opacity: f32 },
    Translate   { x: f32, y: f32 },
    Scale       { sx: f32, sy: f32 },
    Rotate      { angle: f32 },
    PushTransform,
    PopTransform,
    FillShadow  { rect: Rect, radius: f32, spread: f32, color: ltk_core::Color },
}

/// A canvas that records draw commands for later GPU submission.
pub struct Canvas {
    pub commands: Vec<DrawCommand>,
}

impl Canvas {
    pub fn new() -> Self { Self { commands: Vec::with_capacity(64) } }
    pub fn clear(&mut self) { self.commands.clear(); }

    pub fn push_clip(&mut self, rect: Rect, radius: f32) { self.commands.push(DrawCommand::PushClip { rect, radius }); }
    pub fn pop_clip(&mut self)  { self.commands.push(DrawCommand::PopClip); }
    pub fn push_transform(&mut self) { self.commands.push(DrawCommand::PushTransform); }
    pub fn pop_transform(&mut self)  { self.commands.push(DrawCommand::PopTransform); }
    pub fn translate(&mut self, x: f32, y: f32) { self.commands.push(DrawCommand::Translate { x, y }); }
    pub fn scale(&mut self, sx: f32, sy: f32) { self.commands.push(DrawCommand::Scale { sx, sy }); }
    pub fn rotate(&mut self, angle: f32) { self.commands.push(DrawCommand::Rotate { angle }); }

    pub fn fill_rect(&mut self, rect: Rect, paint: Paint) { self.commands.push(DrawCommand::FillRect { rect, paint }); }
    pub fn stroke_rect(&mut self, rect: Rect, paint: Paint, width: f32) { self.commands.push(DrawCommand::StrokeRect { rect, paint, width }); }
    pub fn fill_rounded_rect(&mut self, rect: Rect, radius: f32, paint: Paint) { self.commands.push(DrawCommand::FillRoundedRect { rect, radius, paint }); }
    pub fn fill_circle(&mut self, center: Point, radius: f32, paint: Paint) { self.commands.push(DrawCommand::FillCircle { center, radius, paint }); }
    pub fn draw_line(&mut self, from: Point, to: Point, paint: Paint, width: f32) { self.commands.push(DrawCommand::DrawLine { from, to, paint, width }); }
    pub fn draw_text(&mut self, x: f32, y: f32, text: impl Into<String>, paint: Paint) { self.commands.push(DrawCommand::DrawText { x, y, text: text.into(), paint }); }
    pub fn draw_image(&mut self, src_id: u64, dst: Rect, src: Option<Rect>, opacity: f32) { self.commands.push(DrawCommand::DrawImage { src_id, dst, src, opacity }); }
    pub fn fill_shadow(&mut self, rect: Rect, radius: f32, spread: f32, color: ltk_core::Color) { self.commands.push(DrawCommand::FillShadow { rect, radius, spread, color }); }

    pub fn command_count(&self) -> usize { self.commands.len() }
}

impl Default for Canvas { fn default() -> Self { Self::new() } }
