//! Accessible keyboard navigation helpers.

use ltk_core::id::WidgetId;
use crate::tree::AccessibilityTree;

/// Landmark navigation types (like screen reader H / T / L keys).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LandmarkKind { Main, Navigation, Search, Banner, ContentInfo, Region, Complementary }

/// Find the next focusable node in the a11y tree.
pub fn next_focusable(
    tree: &AccessibilityTree,
    current: Option<WidgetId>,
) -> Option<WidgetId> {
    let root = tree.root()?;
    let mut found_current = current.is_none();
    let mut result = None;
    visit_focusable(tree, root, &mut found_current, &mut |id| {
        if result.is_none() { result = Some(id); }
    });
    result
}

fn visit_focusable(
    tree:   &AccessibilityTree,
    id:     WidgetId,
    found:  &mut bool,
    accept: &mut impl FnMut(WidgetId),
) {
    if let Some(node) = tree.get(id) {
        if *found && node.is_focusable() && node.is_enabled() { accept(id); }
        let children = node.children.clone();
        for child in children { visit_focusable(tree, child, found, accept); }
    }
}
