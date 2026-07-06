//! Finite-state animation machine: discrete states + animated transitions.

use ltk_design::motion::Easing;
use std::collections::HashMap;

/// A named animation state.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnimState(pub String);

/// A transition rule between two states.
#[derive(Debug, Clone)]
pub struct AnimTransition {
    pub from:     AnimState,
    pub to:       AnimState,
    pub duration: std::time::Duration,
    pub easing:   Easing,
}

/// A state machine that drives property values through animated transitions.
pub struct StateMachine {
    pub current:     AnimState,
    pub transitions: Vec<AnimTransition>,
    pub properties:  HashMap<String, f32>,  // property name → current value
}

impl StateMachine {
    pub fn new(initial: impl Into<String>) -> Self {
        Self {
            current:     AnimState(initial.into()),
            transitions: Vec::new(),
            properties:  HashMap::new(),
        }
    }

    pub fn add_transition(mut self, t: AnimTransition) -> Self {
        self.transitions.push(t); self
    }

    /// Trigger a transition to a new state. Returns the matching transition, if any.
    pub fn go_to(&mut self, state: impl Into<String>) -> Option<&AnimTransition> {
        let target = AnimState(state.into());
        let idx = self.transitions.iter().position(|t| t.from == self.current && t.to == target)?;
        self.current = target;
        Some(&self.transitions[idx])
    }
}
