//! Assert accessibility tree shape, roles, and labels.

use ltk_a11y::tree::{AccessibilityTree, AccessibleRole, AccessibleNode};
use ltk_core::id::WidgetId;

pub struct A11yAssertions<'t> { pub tree: &'t AccessibilityTree }

impl<'t> A11yAssertions<'t> {
    pub fn new(tree: &'t AccessibilityTree) -> Self { Self { tree } }

    pub fn assert_role(&self, id: WidgetId, expected: AccessibleRole) -> Result<(), String> {
        let node = self.tree.get(id).ok_or_else(|| format!("Widget {id:?} not in a11y tree"))?;
        if node.role != expected {
            return Err(format!("Expected role {:?}, got {:?}", expected, node.role));
        }
        Ok(())
    }

    pub fn assert_has_name(&self, id: WidgetId) -> Result<(), String> {
        let node = self.tree.get(id).ok_or_else(|| format!("Widget {id:?} not in a11y tree"))?;
        if node.name.is_none() || node.name.as_ref().unwrap().is_empty() {
            return Err(format!("Widget {id:?} has no accessible name"));
        }
        Ok(())
    }

    pub fn assert_focusable(&self, id: WidgetId) -> Result<(), String> {
        let node = self.tree.get(id).ok_or_else(|| format!("Widget {id:?} not in a11y tree"))?;
        if !node.is_focusable() { return Err(format!("Widget {id:?} is not focusable")); }
        Ok(())
    }

    /// Walk the whole tree and assert every interactive widget has a name.
    pub fn assert_all_named(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if let Some(root) = self.tree.root() {
            self.walk(root, &mut errors);
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }

    fn walk(&self, id: WidgetId, errors: &mut Vec<String>) {
        if let Some(node) = self.tree.get(id) {
            if node.is_focusable() && node.name.is_none() {
                errors.push(format!("{:?} ({:?}) has no name", id, node.role));
            }
            for child in node.children.clone() { self.walk(child, errors); }
        }
    }
}
