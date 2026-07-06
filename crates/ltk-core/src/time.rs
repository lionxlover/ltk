//! Time primitives: Duration, Instant, FrameTime.

pub use std::time::{Duration, Instant};

/// Timing information for one rendered frame.
#[derive(Debug, Clone, Copy)]
pub struct FrameTime {
    /// Monotonic timestamp at the start of this frame.
    pub now:      Instant,
    /// Elapsed time since the previous frame (clamped to ≤100ms to handle pauses).
    pub delta:    Duration,
    /// Elapsed time since the application started.
    pub elapsed:  Duration,
    /// The target frame duration at the current refresh rate.
    pub budget:   Duration,
    /// Frame sequence number (wraps after u64::MAX).
    pub frame_nr: u64,
}

impl FrameTime {
    /// Delta in seconds as f32 (convenient for animation math).
    #[inline] pub fn dt(&self) -> f32 { self.delta.as_secs_f32() }
    /// Returns true if we are within frame budget (not late).
    #[inline] pub fn in_budget(&self) -> bool { self.delta <= self.budget }
}
