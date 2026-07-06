//! Stagger: sequential/offset animations for lists and grids.

use std::time::Duration;

/// Config for staggered list entry animations.
#[derive(Debug, Clone)]
pub struct StaggerConfig {
    pub base_delay:  Duration,
    pub per_item:    Duration,
    pub max_delay:   Option<Duration>,
    pub from_center: bool,
}

impl StaggerConfig {
    pub fn new(base: Duration, per_item: Duration) -> Self {
        Self { base_delay: base, per_item, max_delay: None, from_center: false }
    }

    /// Compute the delay for item at `index` in a list of `total` items.
    pub fn delay_for(&self, index: usize, total: usize) -> Duration {
        let i = if self.from_center {
            let center = total as isize / 2;
            (index as isize - center).unsigned_abs()
        } else { index };
        let delay = self.base_delay + self.per_item * i as u32;
        match self.max_delay { Some(max) => delay.min(max), None => delay }
    }
}
