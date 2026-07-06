//! Live region announcements to screen readers.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnnouncePriority { Polite, Assertive }

pub struct LiveAnnouncer { queue: std::sync::Mutex<Vec<(String, AnnouncePriority)>> }

impl LiveAnnouncer {
    pub fn new() -> Self { Self { queue: std::sync::Mutex::new(Vec::new()) } }

    pub fn announce(&self, text: &str, priority: AnnouncePriority) {
        self.queue.lock().unwrap().push((text.to_string(), priority));
        log::debug!("A11y announce [{:?}]: {}", priority, text);
        // Real impl: emit AT-SPI2 `object:announcement` event via D-Bus.
    }

    pub fn drain(&self) -> Vec<(String, AnnouncePriority)> {
        std::mem::take(&mut self.queue.lock().unwrap())
    }
}

impl Default for LiveAnnouncer { fn default() -> Self { Self::new() } }
