//! Headless test runtime: tick/pump helpers without a real display.

use ltk_core::time::{Duration, FrameTime, Instant};

/// Runs the application loop deterministically for tests.
pub struct TestHarness {
    elapsed:  Duration,
    frame_nr: u64,
}

impl TestHarness {
    pub fn new() -> Self { Self { elapsed: Duration::ZERO, frame_nr: 0 } }

    /// Advance the simulated clock and produce one `FrameTime`.
    pub fn pump(&mut self, dt: Duration) -> FrameTime {
        self.elapsed += dt;
        self.frame_nr += 1;
        FrameTime {
            now: Instant::now(),
            delta: dt,
            elapsed: self.elapsed,
            budget: Duration::from_millis(16),
            frame_nr: self.frame_nr,
        }
    }

    /// Pump enough frames to cover `total`, at `step` increments.
    pub fn pump_for(&mut self, total: Duration, step: Duration) -> Vec<FrameTime> {
        let mut frames = Vec::new();
        let mut t = Duration::ZERO;
        while t < total {
            frames.push(self.pump(step));
            t += step;
        }
        frames
    }
}

impl Default for TestHarness { fn default() -> Self { Self::new() } }
