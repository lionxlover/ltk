//! Frame scheduler: VSync alignment and frame budgeting.

use ltk_core::time::{Duration, Instant};

pub struct FrameScheduler {
    pub target_fps:      f32,
    pub adaptive:        bool,
    pub last_frame_time: Option<Instant>,
}

impl FrameScheduler {
    pub fn new(target_fps: f32) -> Self {
        Self { target_fps, adaptive: true, last_frame_time: None }
    }
    pub fn frame_budget(&self) -> Duration {
        Duration::from_secs_f32(1.0 / self.target_fps)
    }
    pub fn should_render(&mut self) -> bool {
        let now    = Instant::now();
        let budget = self.frame_budget();
        match self.last_frame_time {
            None    => { self.last_frame_time = Some(now); true }
            Some(t) => {
                if now.duration_since(t) >= budget { self.last_frame_time = Some(now); true }
                else { false }
            }
        }
    }
}

impl Default for FrameScheduler { fn default() -> Self { Self::new(60.0) } }
