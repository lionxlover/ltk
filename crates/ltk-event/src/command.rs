//! Command pattern: named, executable, undoable operations.

use ltk_core::{error::LtkResult, id::CommandId, string::SharedString};
use std::{collections::HashMap, sync::Arc};

/// Result of executing or undoing a command.
pub type CommandResult = LtkResult<()>;

/// Context passed to commands during execution.
pub struct CommandContext {
    pub widget_id: Option<ltk_core::id::WidgetId>,
    pub extras:    HashMap<String, String>,
}

impl CommandContext {
    pub fn new() -> Self { Self { widget_id: None, extras: HashMap::new() } }
}

impl Default for CommandContext { fn default() -> Self { Self::new() } }

/// A reversible, named operation.
pub trait Command: Send + Sync + 'static {
    fn name(&self) -> &str;
    fn description(&self) -> Option<&str> { None }
    fn execute(&self, ctx: &mut CommandContext) -> CommandResult;
    fn undo(&self, ctx: &mut CommandContext) -> CommandResult;
    fn is_undoable(&self) -> bool { true }
}

type ArcCommand = Arc<dyn Command>;

/// Registry of named commands that can be invoked by ID or name.
pub struct CommandRegistry {
    by_name: HashMap<SharedString, ArcCommand>,
}

impl CommandRegistry {
    pub fn new() -> Self { Self { by_name: HashMap::new() } }

    pub fn register(&mut self, cmd: impl Command) {
        let name = SharedString::new(cmd.name());
        self.by_name.insert(name, Arc::new(cmd));
    }

    pub fn get(&self, name: &str) -> Option<&ArcCommand> {
        self.by_name.get(&SharedString::new(name))
    }

    pub fn execute(&self, name: &str, ctx: &mut CommandContext) -> CommandResult {
        let cmd = self.get(name)
            .ok_or_else(|| ltk_core::error::LtkError::ResourceNotFound { name: name.into() })?;
        cmd.execute(ctx)
    }
}

impl Default for CommandRegistry { fn default() -> Self { Self::new() } }
