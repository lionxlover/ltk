//! Time-travel state snapshots (for DevTools and undo).

/// A snapshot of a serialisable state at a point in time.
pub struct StateSnapshot {
    pub label:     String,
    pub timestamp: std::time::Instant,
    pub data:      serde_json::Value,
}

pub struct StateHistory {
    snapshots: Vec<StateSnapshot>,
    max:       usize,
}

impl StateHistory {
    pub fn new(max: usize) -> Self { Self { snapshots: Vec::new(), max } }

    pub fn push(&mut self, label: impl Into<String>, data: serde_json::Value) {
        self.snapshots.push(StateSnapshot { label: label.into(), timestamp: std::time::Instant::now(), data });
        if self.snapshots.len() > self.max { self.snapshots.remove(0); }
    }

    pub fn len(&self) -> usize { self.snapshots.len() }
    pub fn iter(&self) -> std::slice::Iter<StateSnapshot> { self.snapshots.iter() }
}
