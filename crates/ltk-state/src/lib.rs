//! # ltk-state  —  Fine-grained reactive system
//!
//! Inspired by SolidJS / MobX:
//! - [`Signal<T>`] — observable value
//! - [`Computed<T>`] — derived value (memoized)
//! - [`Effect`] — side-effect that re-runs on dependency change
//! - [`Binding`] — two-way sync between two signals

pub mod signal;
pub mod computed;
pub mod effect;
pub mod binding;
pub mod store;
pub mod history;
pub mod persistence;

pub use signal::Signal;
pub use computed::Computed;
pub use effect::{Effect, EffectHandle};
pub use binding::{Binding, BindingHandle};
