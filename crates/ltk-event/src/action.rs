//! Named UI actions: label, icon, shortcut, enabled state.

use ltk_core::{id::WidgetId, string::SharedString};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ActionId(u64);
static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
impl ActionId { pub fn new() -> Self { Self(NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)) } }

/// A named, triggerable UI action (like a menu item, toolbar button, or shortcut).
pub struct Action {
    pub id:          ActionId,
    pub name:        SharedString,
    pub label:       SharedString,
    pub description: Option<SharedString>,
    pub shortcut:    Option<SharedString>,
    pub enabled:     bool,
    pub checked:     Option<bool>,   // None = not checkable
    pub handler:     Arc<dyn Fn() + Send + Sync>,
}

impl Action {
    pub fn new(name: impl Into<String>, label: impl Into<String>, handler: impl Fn() + Send + Sync + 'static) -> Self {
        Self {
            id:          ActionId::new(),
            name:        SharedString::new(name),
            label:       SharedString::new(label),
            description: None,
            shortcut:    None,
            enabled:     true,
            checked:     None,
            handler:     Arc::new(handler),
        }
    }

    pub fn with_shortcut(mut self, s: impl Into<String>) -> Self { self.shortcut = Some(SharedString::new(s)); self }
    pub fn with_description(mut self, d: impl Into<String>) -> Self { self.description = Some(SharedString::new(d)); self }
    pub fn disabled(mut self) -> Self { self.enabled = false; self }
    pub fn checkable(mut self, checked: bool) -> Self { self.checked = Some(checked); self }

    pub fn trigger(&self) { if self.enabled { (self.handler)(); } }
}
