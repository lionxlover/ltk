//! Vector path operations: bezier curves, boolean ops, stroke expansion.

use ltk_core::geometry::Point;

#[derive(Debug, Clone)]
pub enum PathCmd {
    MoveTo(Point),
    LineTo(Point),
    CubicTo(Point, Point, Point),  // cp1, cp2, end
    QuadTo(Point, Point),           // cp, end
    Close,
}

#[derive(Debug, Clone, Default)]
pub struct VectorPath { pub commands: Vec<PathCmd> }

impl VectorPath {
    pub fn new() -> Self { Self::default() }
    pub fn move_to(mut self, p: Point) -> Self { self.commands.push(PathCmd::MoveTo(p)); self }
    pub fn line_to(mut self, p: Point) -> Self { self.commands.push(PathCmd::LineTo(p)); self }
    pub fn cubic_to(mut self, cp1: Point, cp2: Point, end: Point) -> Self {
        self.commands.push(PathCmd::CubicTo(cp1, cp2, end)); self
    }
    pub fn close(mut self) -> Self { self.commands.push(PathCmd::Close); self }
    pub fn is_empty(&self) -> bool { self.commands.is_empty() }
    pub fn command_count(&self) -> usize { self.commands.len() }
}
