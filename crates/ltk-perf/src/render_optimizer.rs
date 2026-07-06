//! Render optimisation: batch detection and overdraw analysis.

use ltk_core::geometry::Rect;

/// Analyzes draw command lists for optimisation opportunities.
pub struct RenderOptimizer;

impl RenderOptimizer {
    /// Estimate overdraw ratio for a set of opaque rects.
    pub fn overdraw_ratio(rects: &[Rect], viewport: Rect) -> f32 {
        let vp_area = viewport.size.area();
        if vp_area <= 0.0 { return 0.0; }
        let total_drawn: f32 = rects.iter().map(|r| r.size.area()).sum();
        total_drawn / vp_area
    }

    /// Find rectangles that are fully occluded by later opaque rects.
    pub fn find_occluded(rects: &[Rect]) -> Vec<usize> {
        let mut occluded = Vec::new();
        for i in 0..rects.len() {
            for j in (i + 1)..rects.len() {
                if rects[j].contains(rects[i].origin) &&
                   rects[j].contains(ltk_core::geometry::Point::new(rects[i].max_x(), rects[i].max_y()))
                {
                    occluded.push(i);
                    break;
                }
            }
        }
        occluded
    }
}
