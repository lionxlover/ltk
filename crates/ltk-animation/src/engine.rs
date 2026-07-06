//! Central animation engine: registry, tick, frame coupling.

use ltk_core::{id::AnimationId, time::{Instant, Duration, FrameTime}};
use ltk_design::motion::{Easing, DurationToken};
use std::collections::HashMap;

/// How many times an animation repeats.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RepeatCount { Times(u32), Infinite }

/// State of a single running animation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationState { Pending, Running, Paused, Completed, Cancelled }

/// A handle to a running animation. Clone cheaply; check state or cancel.
#[derive(Debug, Clone)]
pub struct AnimationHandle {
    pub id:    AnimationId,
    state:     std::sync::Arc<std::sync::atomic::AtomicU8>,
}

impl AnimationHandle {
    pub fn is_running(&self) -> bool {
        self.state.load(std::sync::atomic::Ordering::Relaxed) == 1
    }
    pub fn cancel(&self) {
        self.state.store(4, std::sync::atomic::Ordering::Relaxed);
    }
}

/// A registered, running animation entry in the engine.
struct AnimEntry {
    id:        AnimationId,
    start:     Instant,
    duration:  Duration,
    easing:    Easing,
    repeat:    RepeatCount,
    state:     AnimationState,
    on_tick:   Box<dyn Fn(f32) + Send + Sync>,   // called with t ∈ [0,1]
    on_done:   Option<Box<dyn Fn() + Send + Sync>>,
}

/// Singleton animation engine (one per application).
pub struct AnimationEngine {
    entries: HashMap<AnimationId, AnimEntry>,
    elapsed_total: Duration,
}

impl AnimationEngine {
    pub fn new() -> Self { Self { entries: HashMap::new(), elapsed_total: Duration::ZERO } }

    /// Register and start a new animation. Returns a handle.
    pub fn start(
        &mut self,
        duration:   Duration,
        easing:     Easing,
        repeat:     RepeatCount,
        on_tick:    impl Fn(f32) + Send + Sync + 'static,
        on_done:    Option<impl Fn() + Send + Sync + 'static>,
    ) -> AnimationHandle {
        let id = AnimationId::new();
        let state_atom = std::sync::Arc::new(std::sync::atomic::AtomicU8::new(1));
        let entry = AnimEntry {
            id, start: Instant::now(), duration, easing, repeat,
            state: AnimationState::Running,
            on_tick: Box::new(on_tick),
            on_done: on_done.map(|f| Box::new(f) as Box<dyn Fn() + Send + Sync>),
        };
        self.entries.insert(id, entry);
        AnimationHandle { id, state: state_atom }
    }

    /// Advance all animations by one frame. Called from the frame loop.
    pub fn tick(&mut self, frame: &FrameTime) {
        self.elapsed_total += frame.delta;
        let now = frame.now;
        let mut completed = Vec::new();

        for (id, entry) in &mut self.entries {
            if entry.state != AnimationState::Running { continue; }
            let elapsed = now.duration_since(entry.start).as_secs_f32();
            let total   = entry.duration.as_secs_f32();
            if total <= 0.0 { (entry.on_tick)(1.0); completed.push(*id); continue; }

            let raw_t = (elapsed / total).clamp(0.0, 1.0);
            let t = entry.easing.evaluate(raw_t);
            (entry.on_tick)(t);

            if raw_t >= 1.0 {
                match entry.repeat {
                    RepeatCount::Times(n) if n <= 1 => {
                        entry.state = AnimationState::Completed;
                        completed.push(*id);
                    }
                    RepeatCount::Times(n) => {
                        entry.repeat = RepeatCount::Times(n - 1);
                        entry.start  = now;
                    }
                    RepeatCount::Infinite => { entry.start = now; }
                }
            }
        }

        for id in &completed {
            if let Some(e) = self.entries.get(id) {
                if let Some(cb) = &e.on_done { cb(); }
            }
            // Keep completed entries for one frame for on_done, then remove
        }
        self.entries.retain(|id, e| !completed.contains(id) || e.state == AnimationState::Running);
    }

    pub fn active_count(&self) -> usize { self.entries.len() }
    pub fn cancel(&mut self, id: AnimationId) {
        if let Some(e) = self.entries.get_mut(&id) { e.state = AnimationState::Cancelled; }
    }
}

impl Default for AnimationEngine { fn default() -> Self { Self::new() } }
