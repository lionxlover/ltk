//! Cursor shape management and custom cursors.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CursorShape {
    Default, Pointer, Text, Crosshair, Move,
    NResize, SResize, EResize, WResize,
    NeResize, NwResize, SeResize, SwResize,
    EwResize, NsResize, NwseResize, NeswResize,
    ColResize, RowResize, AllScroll,
    ZoomIn, ZoomOut, Grab, Grabbing,
    Copy, Alias, NoDrop, NotAllowed,
    Progress, Wait,
    Help, Cell, VerticalText,
    None,
}

pub struct CursorManager { pub current: CursorShape }

impl CursorManager {
    pub fn new() -> Self { Self { current: CursorShape::Default } }
    pub fn set(&mut self, shape: CursorShape) { self.current = shape; }
    pub fn reset(&mut self) { self.current = CursorShape::Default; }
}

impl Default for CursorManager { fn default() -> Self { Self::new() } }
