//! xdg-decoration protocol: server-side vs client-side decoration negotiation.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecorationMode { ClientSide, ServerSide }

pub struct DecorationManager { pub preferred: DecorationMode }

impl DecorationManager {
    pub fn new() -> Self {
        // LTK always prefers drawing its own titlebar for design consistency.
        Self { preferred: DecorationMode::ClientSide }
    }

    pub fn request_mode(&self, mode: DecorationMode) {
        log::debug!("Decoration: requesting {mode:?}");
    }
}

impl Default for DecorationManager { fn default() -> Self { Self::new() } }
