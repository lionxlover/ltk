//! Live widget tree inspector (like GTK Inspector / Browser DevTools).

use ltk_core::id::WidgetId;
use ltk_core::geometry::Rect;

#[derive(Debug, Clone)]
pub struct InspectedWidget {
    pub id:         WidgetId,
    pub type_name:  String,
    pub bounds:     Rect,
    pub properties: Vec<(String, String)>,
    pub children:   Vec<WidgetId>,
}

/// The inspector tool: walks the live widget/layout/scene trees.
pub struct WidgetInspector {
    pub selected:   Option<WidgetId>,
    pub hover:      Option<WidgetId>,
    pub picking:    bool,    // "click to select element" mode active
}

impl WidgetInspector {
    pub fn new() -> Self { Self { selected: None, hover: None, picking: false } }

    pub fn select(&mut self, id: WidgetId) { self.selected = Some(id); }
    pub fn start_picking(&mut self) { self.picking = true; }
    pub fn stop_picking(&mut self)  { self.picking = false; }

    /// Find the topmost widget at a point (hit-testing for picker mode).
    pub fn hit_test(&self, point: ltk_core::geometry::Point, widgets: &[InspectedWidget]) -> Option<WidgetId> {
        widgets.iter().rev()
            .find(|w| w.bounds.contains(point))
            .map(|w| w.id)
    }
}

impl Default for WidgetInspector { fn default() -> Self { Self::new() } }
