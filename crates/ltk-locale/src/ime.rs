//! Input Method Editor bridge (IBus/Fcitx5).

/// State of an active IME composition.
#[derive(Debug, Clone, Default)]
pub struct ImeComposition {
    pub text:      String,    // pre-edit text
    pub cursor:    usize,     // cursor position in pre-edit
    pub committed: Option<String>, // finalized text when composition ends
}

/// Events from the IME.
#[derive(Debug, Clone)]
pub enum ImeEvent {
    CompositionStart,
    CompositionUpdate(ImeComposition),
    CompositionEnd { committed: String },
    CandidateList  { candidates: Vec<String>, page: usize },
}

/// Interface for an IME backend (IBus, Fcitx5, or the system input method).
pub trait ImeBackend: Send + Sync {
    fn set_cursor_position(&self, x: i32, y: i32);
    fn set_surrounding_text(&self, text: &str, cursor: usize);
    fn commit(&self);
    fn cancel(&self);
    fn is_active(&self) -> bool;
}

/// Stub IME backend for systems without a running IM.
pub struct NullImeBackend;

impl ImeBackend for NullImeBackend {
    fn set_cursor_position(&self, _: i32, _: i32) {}
    fn set_surrounding_text(&self, _: &str, _: usize) {}
    fn commit(&self) {}
    fn cancel(&self) {}
    fn is_active(&self) -> bool { false }
}
