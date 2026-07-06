//! Motion path: animate an element along a Bezier or SVG path.

use ltk_core::geometry::Point;

/// A cubic Bezier segment (4 control points).
#[derive(Debug, Clone, Copy)]
pub struct CubicBezier { pub p0: Point, pub p1: Point, pub p2: Point, pub p3: Point }

impl CubicBezier {
    /// Evaluate the position on the curve at parameter `t` ∈ [0,1].
    pub fn evaluate(&self, t: f32) -> Point {
        let u = 1.0 - t;
        let tt = t * t;
        let uu = u * u;
        let uuu = uu * u;
        let ttt = tt * t;
        Point {
            x: uuu*self.p0.x + 3.0*uu*t*self.p1.x + 3.0*u*tt*self.p2.x + ttt*self.p3.x,
            y: uuu*self.p0.y + 3.0*uu*t*self.p1.y + 3.0*u*tt*self.p2.y + ttt*self.p3.y,
        }
    }

    /// Estimate the tangent direction at `t` (normalised).
    pub fn tangent(&self, t: f32) -> Point {
        let u  = 1.0 - t;
        let dx = 3.0*(u*u*(self.p1.x-self.p0.x) + 2.0*u*t*(self.p2.x-self.p1.x) + t*t*(self.p3.x-self.p2.x));
        let dy = 3.0*(u*u*(self.p1.y-self.p0.y) + 2.0*u*t*(self.p2.y-self.p1.y) + t*t*(self.p3.y-self.p2.y));
        let len = (dx*dx + dy*dy).sqrt().max(1e-6);
        Point { x: dx/len, y: dy/len }
    }
}

/// A compound motion path made of Bezier segments.
#[derive(Debug, Clone, Default)]
pub struct MotionPath { pub segments: Vec<CubicBezier> }

impl MotionPath {
    pub fn evaluate(&self, t: f32) -> Point {
        if self.segments.is_empty() { return Point::ZERO; }
        let n = self.segments.len() as f32;
        let seg_t = (t * n).floor().min(n - 1.0) as usize;
        let local_t = (t * n) - seg_t as f32;
        self.segments[seg_t].evaluate(local_t)
    }
}
