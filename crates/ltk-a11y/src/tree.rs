//! Accessibility node tree that mirrors the widget tree.

use ltk_core::{id::WidgetId, string::SharedString};
use std::collections::HashMap;
use bitflags::bitflags;
use serde::{Deserialize, Serialize};

/// ARIA / AT-SPI2 role.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AccessibleRole {
    None, Button, CheckBox, ComboBox, Dialog, Document,
    Heading, Image, Label, Link, List, ListItem,
    Menu, MenuBar, MenuItem, Option, ProgressBar,
    RadioButton, ScrollBar, Slider, SpinButton,
    StatusBar, Tab, TabList, TabPanel, TextBox, TextArea,
    ToggleButton, ToolBar, ToolTip, Tree, TreeItem,
    Window, Group, Row, Cell, ColumnHeader, RowHeader,
    Landmark, Region, Navigation, Main, Search, Banner,
    ContentInfo, Complementary, Form, Application,
    Custom(String),
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct AccessibleState: u64 {
        const BUSY         = 0x0001;
        const CHECKED      = 0x0002;
        const DISABLED     = 0x0004;
        const EDITABLE     = 0x0008;
        const ENABLED      = 0x0010;
        const EXPANDED     = 0x0020;
        const FOCUSABLE    = 0x0040;
        const FOCUSED      = 0x0080;
        const MODAL        = 0x0100;
        const MULTILINE    = 0x0200;
        const MULTISELECT  = 0x0400;
        const PRESSED      = 0x0800;
        const REQUIRED     = 0x1000;
        const SELECTED     = 0x2000;
        const VISIBLE      = 0x4000;
        const INVALID      = 0x8000;
        const READONLY     = 0x0001_0000;
    }
}

/// An action that can be invoked on an accessible node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibleAction {
    pub name:        String,
    pub description: Option<String>,
    pub key_binding: Option<String>,
}

/// Value associated with an accessible node (for sliders, spinners, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibleValue {
    pub current: f64,
    pub min:     f64,
    pub max:     f64,
    pub step:    f64,
}

/// A single node in the accessibility tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibleNode {
    pub id:          WidgetId,
    pub role:        AccessibleRole,
    pub name:        Option<SharedString>,
    pub description: Option<SharedString>,
    pub value:       Option<AccessibleValue>,
    pub state:       AccessibleState,
    pub actions:     Vec<AccessibleAction>,
    pub children:    Vec<WidgetId>,
    pub parent:      Option<WidgetId>,
    pub bounds:      ltk_core::geometry::Rect,
    pub text_value:  Option<SharedString>,
    pub level:       Option<u32>,    // heading level, tree depth
    pub set_size:    Option<u32>,    // total items in group
    pub pos_in_set:  Option<u32>,    // position within group
}

impl AccessibleNode {
    pub fn new(id: WidgetId, role: AccessibleRole) -> Self {
        Self {
            id, role, name: None, description: None, value: None,
            state: AccessibleState::ENABLED | AccessibleState::VISIBLE,
            actions: Vec::new(), children: Vec::new(), parent: None,
            bounds: ltk_core::geometry::Rect::ZERO, text_value: None,
            level: None, set_size: None, pos_in_set: None,
        }
    }

    pub fn is_focusable(&self) -> bool { self.state.contains(AccessibleState::FOCUSABLE) }
    pub fn is_focused(&self)   -> bool { self.state.contains(AccessibleState::FOCUSED) }
    pub fn is_enabled(&self)   -> bool { self.state.contains(AccessibleState::ENABLED) }
    pub fn is_visible(&self)   -> bool { self.state.contains(AccessibleState::VISIBLE) }
}

/// The full accessibility tree for one window.
pub struct AccessibilityTree {
    nodes:  HashMap<WidgetId, AccessibleNode>,
    root:   Option<WidgetId>,
    dirty:  Vec<WidgetId>,
}

impl AccessibilityTree {
    pub fn new() -> Self { Self { nodes: HashMap::new(), root: None, dirty: Vec::new() } }

    pub fn set_root(&mut self, id: WidgetId) { self.root = Some(id); }
    pub fn root(&self) -> Option<WidgetId> { self.root }

    pub fn insert(&mut self, node: AccessibleNode) {
        let id = node.id;
        self.nodes.insert(id, node);
        self.dirty.push(id);
    }

    pub fn get(&self, id: WidgetId) -> Option<&AccessibleNode> { self.nodes.get(&id) }
    pub fn get_mut(&mut self, id: WidgetId) -> Option<&mut AccessibleNode> { self.nodes.get_mut(&id) }

    pub fn update_state(&mut self, id: WidgetId, state: AccessibleState) {
        if let Some(n) = self.nodes.get_mut(&id) {
            n.state = state;
            self.dirty.push(id);
        }
    }

    pub fn drain_dirty(&mut self) -> Vec<WidgetId> { std::mem::take(&mut self.dirty) }
    pub fn node_count(&self) -> usize { self.nodes.len() }
}

impl Default for AccessibilityTree { fn default() -> Self { Self::new() } }
