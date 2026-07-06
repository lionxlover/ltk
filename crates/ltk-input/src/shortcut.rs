//! Keyboard shortcut matching and conflict detection.

use crate::keyboard::{KeyCode, Modifiers};
use std::collections::HashMap;

/// A keyboard shortcut (modifier combination + key code).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Shortcut {
    pub modifiers: Modifiers,
    pub key:       KeyCode,
}

impl Shortcut {
    pub fn new(mods: Modifiers, key: KeyCode) -> Self { Self { modifiers: mods, key } }
    pub fn ctrl(key: KeyCode) -> Self { Self::new(Modifiers::CTRL, key) }
    pub fn ctrl_shift(key: KeyCode) -> Self { Self::new(Modifiers::CTRL | Modifiers::SHIFT, key) }
    pub fn alt(key: KeyCode) -> Self { Self::new(Modifiers::ALT, key) }

    pub fn matches(&self, event: &crate::keyboard::KeyEvent) -> bool {
        self.key == event.code && self.modifiers == event.modifiers && event.is_press()
    }
}

impl std::fmt::Display for Shortcut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.modifiers.contains(Modifiers::CTRL)  { write!(f, "Ctrl+")?; }
        if self.modifiers.contains(Modifiers::SHIFT) { write!(f, "Shift+")?; }
        if self.modifiers.contains(Modifiers::ALT)   { write!(f, "Alt+")?; }
        if self.modifiers.contains(Modifiers::SUPER) { write!(f, "Super+")?; }
        write!(f, "{:?}", self.key)
    }
}

/// Registry mapping shortcuts to named action IDs.
pub struct ShortcutManager {
    bindings: HashMap<Shortcut, String>,   // shortcut → action name
}

impl ShortcutManager {
    pub fn new() -> Self { Self { bindings: HashMap::new() } }

    pub fn register(&mut self, shortcut: Shortcut, action: impl Into<String>) {
        self.bindings.insert(shortcut, action.into());
    }

    /// Returns the action name matching this key event, if any.
    pub fn match_event(&self, event: &crate::keyboard::KeyEvent) -> Option<&str> {
        self.bindings.iter()
            .find_map(|(s, a)| if s.matches(event) { Some(a.as_str()) } else { None })
    }

    pub fn has_conflict(&self, shortcut: &Shortcut) -> bool {
        self.bindings.contains_key(shortcut)
    }
}

impl Default for ShortcutManager { fn default() -> Self { Self::new() } }
