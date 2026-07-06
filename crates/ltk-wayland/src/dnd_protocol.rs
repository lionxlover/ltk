//! wl_data_device protocol: drag-and-drop and clipboard data exchange.

#[derive(Debug, Clone)]
pub struct DataOffer { pub mime_types: Vec<String> }

pub struct DataDeviceManager;

impl DataDeviceManager {
    pub fn start_drag(&self, mime_types: &[String]) {
        log::debug!("Wayland DnD: start_drag({mime_types:?})");
    }
    pub fn set_selection(&self, mime_types: &[String]) {
        log::debug!("Wayland clipboard: set_selection({mime_types:?})");
    }
}
