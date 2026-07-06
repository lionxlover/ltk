//! # ltk-animation — Timeline, Spring, Physics, State Machines

pub mod engine;
pub mod timeline;
pub mod transition;
pub mod spring;
pub mod physics;
pub mod stagger;
pub mod motion_path;
pub mod state_machine;
pub mod frame_sync;

pub use engine::{AnimationEngine, AnimationHandle, RepeatCount};
pub use timeline::{Timeline, Keyframe};
pub use spring::{SpringConfig, SpringSim, SpringHandle};
pub use state_machine::{StateMachine, AnimState, AnimTransition};
pub use transition::Transition;
