//! Style transitions: property changes on widget state switches.

use ltk_design::motion::Easing;
use std::time::Duration;

/// Specifies how a property transition should play.
#[derive(Debug, Clone)]
pub struct Transition {
    pub property: String,
    pub duration: Duration,
    pub easing:   Easing,
    pub delay:    Duration,
}

impl Transition {
    pub fn new(property: impl Into<String>, duration: Duration, easing: Easing) -> Self {
        Self { property: property.into(), duration, easing, delay: Duration::ZERO }
    }
    pub fn with_delay(mut self, delay: Duration) -> Self { self.delay = delay; self }
    pub const DEFAULT_MS: u64 = 200;
}
