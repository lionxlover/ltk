//! # ltk-event  —  Event Bus · Signal/Slot · Commands · Undo/Redo

pub mod bus;
pub mod signal;
pub mod command;
pub mod undo;
pub mod action;
pub mod dispatcher;

pub use bus::{EventBus, SubscriptionId};
pub use signal::{Signal, SlotHandle};
pub use command::{Command, CommandContext, CommandResult, CommandRegistry};
pub use undo::{UndoStack, UndoEvent};
pub use action::{Action, ActionId};
