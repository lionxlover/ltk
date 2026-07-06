//! Effect: a side-effect that re-runs when its signal dependencies change.

use std::sync::Arc;

/// A running effect handle. Drop to stop the effect.
pub struct EffectHandle(Arc<()>);

impl EffectHandle {
    pub fn is_alive(&self) -> bool { Arc::strong_count(&self.0) > 1 }
}

/// Create an effect that runs `f` immediately and re-runs whenever a
/// dependency signal changes. Returns a handle; drop to cancel.
pub fn effect(f: impl Fn() + Send + Sync + 'static) -> EffectHandle {
    f(); // run immediately
    // In a full implementation, track signal reads inside `f` and
    // re-run on any change. Requires thread-local dependency tracking.
    EffectHandle(Arc::new(()))
}

/// Opaque effect identifier.
pub struct Effect { pub(crate) _handle: EffectHandle }

impl Effect {
    pub fn new(f: impl Fn() + Send + Sync + 'static) -> Self {
        Self { _handle: effect(f) }
    }
}
