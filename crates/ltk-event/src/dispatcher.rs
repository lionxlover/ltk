//! Event dispatcher: capture → target → bubble phases.

use ltk_core::id::WidgetId;
use std::collections::VecDeque;

/// Phase of event propagation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropagationPhase { Capture, Target, Bubble }

/// Result of an event handler: continue or stop propagation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropagationResult { Continue, StopPropagation, StopImmediatePropagation }

/// A pending event in the dispatcher queue.
pub struct PendingEvent {
    pub target:  WidgetId,
    pub payload: Box<dyn std::any::Any + Send>,
}

/// Routes events through the widget hierarchy in three phases.
pub struct EventDispatcher {
    queue: VecDeque<PendingEvent>,
}

impl EventDispatcher {
    pub fn new() -> Self { Self { queue: VecDeque::new() } }

    /// Enqueue an event directed at a specific target widget.
    pub fn enqueue<E: 'static + Send>(&mut self, target: WidgetId, event: E) {
        self.queue.push_back(PendingEvent { target, payload: Box::new(event) });
    }

    pub fn is_empty(&self) -> bool { self.queue.is_empty() }
    pub fn len(&self) -> usize { self.queue.len() }
    pub fn drain(&mut self) -> impl Iterator<Item = PendingEvent> + '_ { self.queue.drain(..) }
}

impl Default for EventDispatcher { fn default() -> Self { Self::new() } }
