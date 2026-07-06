//! Drag-and-drop: source, target, MIME negotiation.

use ltk_core::geometry::Point;
use ltk_core::id::WidgetId;

/// A MIME type string.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MimeType(pub String);

impl MimeType {
    pub fn text_plain()     -> Self { Self("text/plain".into()) }
    pub fn text_uri_list()  -> Self { Self("text/uri-list".into()) }
    pub fn image_png()      -> Self { Self("image/png".into()) }
    pub fn app_json()       -> Self { Self("application/json".into()) }
}

/// Data offered by a drag source.
#[derive(Debug, Clone)]
pub struct DragOffer {
    pub source: WidgetId,
    pub types:  Vec<MimeType>,
    pub data:   Vec<u8>,
}

/// A drag-and-drop event.
#[derive(Debug, Clone)]
pub struct DndEvent {
    pub kind:     DndEventKind,
    pub position: Point,
    pub offer:    Option<DragOffer>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DndEventKind { Started, Entered, Moved, Left, Dropped, Cancelled }

/// Capabilities of a drag source.
pub struct DndSource {
    pub widget:   WidgetId,
    pub types:    Vec<MimeType>,
}

/// Capabilities of a drop target.
pub struct DndTarget {
    pub widget:   WidgetId,
    pub accepts:  Vec<MimeType>,
}

impl DndTarget {
    pub fn accepts_type(&self, mime: &MimeType) -> bool { self.accepts.contains(mime) }
}
