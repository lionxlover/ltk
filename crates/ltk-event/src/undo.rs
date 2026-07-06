//! UndoStack with linear history and branching support.

use crate::command::{ArcCommand, CommandContext, CommandResult};
use ltk_core::id::CommandId;

pub struct UndoEvent { pub kind: UndoKind, pub command_name: String }
pub enum UndoKind { Executed, Undone, Redone, StackCleared }

/// Linear undo stack with configurable depth limit.
pub struct UndoStack {
    past:       Vec<ArcCommand>,
    future:     Vec<ArcCommand>,
    max_depth:  usize,
}

impl UndoStack {
    pub fn new(max_depth: usize) -> Self {
        Self { past: Vec::new(), future: Vec::new(), max_depth }
    }

    /// Push a command onto the stack and execute it.
    pub fn execute(
        &mut self,
        cmd:  ArcCommand,
        ctx:  &mut CommandContext,
    ) -> CommandResult {
        cmd.execute(ctx)?;
        self.future.clear();
        self.past.push(cmd);
        if self.past.len() > self.max_depth {
            self.past.remove(0);
        }
        Ok(())
    }

    pub fn can_undo(&self) -> bool { !self.past.is_empty() }
    pub fn can_redo(&self) -> bool { !self.future.is_empty() }

    pub fn undo(&mut self, ctx: &mut CommandContext) -> CommandResult {
        let cmd = self.past.pop()
            .ok_or_else(|| ltk_core::error::LtkError::internal("nothing to undo"))?;
        cmd.undo(ctx)?;
        self.future.push(cmd);
        Ok(())
    }

    pub fn redo(&mut self, ctx: &mut CommandContext) -> CommandResult {
        let cmd = self.future.pop()
            .ok_or_else(|| ltk_core::error::LtkError::internal("nothing to redo"))?;
        cmd.execute(ctx)?;
        self.past.push(cmd);
        Ok(())
    }

    pub fn clear(&mut self) { self.past.clear(); self.future.clear(); }
    pub fn depth(&self) -> usize { self.past.len() }
    pub fn undo_label(&self) -> Option<&str> { self.past.last().map(|c| c.name()) }
    pub fn redo_label(&self) -> Option<&str> { self.future.last().map(|c| c.name()) }
}

impl Default for UndoStack { fn default() -> Self { Self::new(100) } }
