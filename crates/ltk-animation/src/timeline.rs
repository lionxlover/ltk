//! Keyframe timeline: animate any f32 property along a curve.

use ltk_design::motion::Easing;

/// A single keyframe on a timeline.
#[derive(Debug, Clone)]
pub struct Keyframe {
    pub offset: f32,     // 0.0–1.0 position in the timeline
    pub value:  f32,
    pub easing: Easing,  // easing TO this keyframe (from the previous one)
}

/// An animation timeline for a single f32 property.
#[derive(Debug, Clone, Default)]
pub struct Timeline {
    pub keyframes: Vec<Keyframe>,
}

impl Timeline {
    pub fn new() -> Self { Self { keyframes: Vec::new() } }

    pub fn add_keyframe(mut self, offset: f32, value: f32, easing: Easing) -> Self {
        self.keyframes.push(Keyframe { offset, value, easing });
        self.keyframes.sort_by(|a, b| a.offset.partial_cmp(&b.offset).unwrap());
        self
    }

    /// Evaluate the timeline at a progress value `t` ∈ [0,1].
    pub fn evaluate(&self, t: f32) -> f32 {
        if self.keyframes.is_empty() { return 0.0; }
        if self.keyframes.len() == 1 { return self.keyframes[0].value; }
        // Find surrounding keyframes
        let i = self.keyframes.partition_point(|k| k.offset < t);
        if i == 0 { return self.keyframes[0].value; }
        if i >= self.keyframes.len() { return self.keyframes.last().unwrap().value; }
        let a = &self.keyframes[i - 1];
        let b = &self.keyframes[i];
        let seg_t = (t - a.offset) / (b.offset - a.offset);
        let eased  = b.easing.evaluate(seg_t);
        a.value + (b.value - a.value) * eased
    }
}
