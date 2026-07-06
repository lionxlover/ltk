//! VSync-coupled frame timing for animations.

use ltk_core::time::{Instant, Duration, FrameTime};

/// Tracks VSync timing and produces `FrameTime` structs.
pub struct FrameSync {
    start:       Instant,
    last_frame:  Instant,
    frame_nr:    u64,
    target_fps:  f32,
}

impl FrameSync {
    pub fn new(target_fps: f32) -> Self {
        let now = Instant::now();
        Self { start: now, last_frame: now, frame_nr: 0, target_fps }
    }

    /// Call once per VSync. Returns the `FrameTime` for this frame.
    pub fn advance(&mut self) -> FrameTime {
        let now     = Instant::now();
        let delta   = now.duration_since(self.last_frame)
            .min(Duration::from_millis(100)); // clamp for pause resumption
        let elapsed = now.duration_since(self.start);
        let budget  = Duration::from_secs_f32(1.0 / self.target_fps);
        self.last_frame = now;
        self.frame_nr  += 1;
        FrameTime { now, delta, elapsed, budget, frame_nr: self.frame_nr }
    }

    pub fn frame_budget(&self) -> Duration {
        Duration::from_secs_f32(1.0 / self.target_fps)
    }
}
