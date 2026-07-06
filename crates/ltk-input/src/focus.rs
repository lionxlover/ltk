//! Focus engine: tab order, focus scopes, keyboard focus management.

use ltk_core::id::WidgetId;
use std::collections::HashMap;

/// How a widget participates in focus traversal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusPolicy {
    /// Cannot receive focus.
    Never,
    /// Focusable via keyboard Tab/Shift-Tab.
    Keyboard,
    /// Focusable via mouse click only.
    Click,
    /// Focusable via any input.
    All,
}

/// A group of focusable widgets (e.g. a modal dialog).
pub struct FocusScope {
    pub id:       WidgetId,
    pub members:  Vec<(WidgetId, i32)>,   // (id, tab_index)
    pub trap:     bool,    // trap focus inside (modal)
    pub restore:  bool,    // restore previous focus on exit
}

impl FocusScope {
    pub fn new(id: WidgetId) -> Self {
        Self { id, members: Vec::new(), trap: false, restore: true }
    }
    pub fn add(&mut self, widget: WidgetId, tab_index: i32) {
        self.members.push((widget, tab_index));
        self.members.sort_by_key(|&(_, i)| i);
    }
    pub fn ordered_ids(&self) -> Vec<WidgetId> { self.members.iter().map(|&(id, _)| id).collect() }
}

/// Central focus engine managing the focused widget and scope stack.
pub struct FocusEngine {
    current:     Option<WidgetId>,
    scope_stack: Vec<FocusScope>,
    policies:    HashMap<WidgetId, FocusPolicy>,
}

impl FocusEngine {
    pub fn new() -> Self {
        Self { current: None, scope_stack: Vec::new(), policies: HashMap::new() }
    }

    pub fn current(&self) -> Option<WidgetId> { self.current }
    pub fn set_policy(&mut self, id: WidgetId, policy: FocusPolicy) { self.policies.insert(id, policy); }
    pub fn can_focus(&self, id: WidgetId) -> bool {
        !matches!(self.policies.get(&id), Some(FocusPolicy::Never))
    }

    pub fn focus(&mut self, id: WidgetId) -> bool {
        if !self.can_focus(id) { return false; }
        self.current = Some(id);
        true
    }

    pub fn blur(&mut self) { self.current = None; }

    /// Move focus to the next widget in tab order.
    pub fn focus_next(&mut self) -> Option<WidgetId> {
        let scope = self.scope_stack.last()?;
        let ids   = scope.ordered_ids();
        if ids.is_empty() { return None; }
        let next = match self.current {
            None     => ids[0],
            Some(id) => {
                let pos = ids.iter().position(|&i| i == id).unwrap_or(ids.len() - 1);
                ids[(pos + 1) % ids.len()]
            }
        };
        self.focus(next);
        self.current
    }

    /// Move focus to the previous widget (Shift+Tab).
    pub fn focus_prev(&mut self) -> Option<WidgetId> {
        let scope = self.scope_stack.last()?;
        let ids   = scope.ordered_ids();
        if ids.is_empty() { return None; }
        let prev = match self.current {
            None     => *ids.last().unwrap(),
            Some(id) => {
                let pos = ids.iter().position(|&i| i == id).unwrap_or(0);
                ids[(pos + ids.len() - 1) % ids.len()]
            }
        };
        self.focus(prev);
        self.current
    }

    pub fn push_scope(&mut self, scope: FocusScope) { self.scope_stack.push(scope); }
    pub fn pop_scope(&mut self) -> Option<FocusScope> { self.scope_stack.pop() }
}

impl Default for FocusEngine { fn default() -> Self { Self::new() } }
