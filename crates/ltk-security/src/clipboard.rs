//! Secure clipboard with type filtering and access control.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClipboardMime(pub String);

impl ClipboardMime {
    pub fn text()  -> Self { Self("text/plain".into()) }
    pub fn html()  -> Self { Self("text/html".into()) }
    pub fn image() -> Self { Self("image/png".into()) }
}

pub struct SecureClipboard {
    content:     HashMap<ClipboardMime, Vec<u8>>,
    allow_html:  bool,
    allow_files: bool,
}

impl SecureClipboard {
    pub fn new() -> Self {
        Self { content: HashMap::new(), allow_html: true, allow_files: false }
    }

    pub fn write(&mut self, mime: ClipboardMime, data: Vec<u8>) {
        // Block certain types based on policy
        if mime == ClipboardMime("text/html".into()) && !self.allow_html { return; }
        self.content.insert(mime, data);
    }

    pub fn read(&self, mime: &ClipboardMime) -> Option<&[u8]> {
        self.content.get(mime).map(|v| v.as_slice())
    }

    pub fn read_text(&self) -> Option<String> {
        self.read(&ClipboardMime::text())
            .and_then(|b| String::from_utf8(b.to_vec()).ok())
    }

    pub fn write_text(&mut self, text: impl Into<String>) {
        self.write(ClipboardMime::text(), text.into().into_bytes());
    }

    pub fn clear(&mut self) { self.content.clear(); }
    pub fn available_types(&self) -> Vec<&ClipboardMime> { self.content.keys().collect() }
}

impl Default for SecureClipboard { fn default() -> Self { Self::new() } }
