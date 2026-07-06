//! XDG Desktop Portal interface for file chooser, screenshot, settings, etc.

use ltk_core::LtkResult;
use std::path::PathBuf;

pub struct XdgPortal;

impl XdgPortal {
    /// Open the file chooser dialog. Returns selected paths.
    pub fn open_file(
        title:    &str,
        multiple: bool,
        filters:  &[(&str, &str)],
    ) -> LtkResult<Vec<PathBuf>> {
        log::info!("XDG Portal: OpenFile(title={:?}, multiple={})", title, multiple);
        // Real impl: call org.freedesktop.portal.FileChooser.OpenFile via D-Bus
        Ok(Vec::new())
    }

    /// Open the save-file dialog.
    pub fn save_file(title: &str, suggested_name: &str) -> LtkResult<Option<PathBuf>> {
        log::info!("XDG Portal: SaveFile(title={:?})", title);
        Ok(None)
    }

    /// Read a system setting via org.freedesktop.portal.Settings.
    pub fn read_setting(namespace: &str, key: &str) -> LtkResult<String> {
        log::debug!("XDG Portal: Settings.Read({}/{})", namespace, key);
        Ok(String::new())
    }

    /// Take a screenshot via the portal.
    pub fn screenshot(interactive: bool) -> LtkResult<Option<PathBuf>> {
        log::info!("XDG Portal: Screenshot(interactive={})", interactive);
        Ok(None)
    }

    /// Open a URI in the default handler.
    pub fn open_uri(uri: &str) -> LtkResult<()> {
        log::info!("XDG Portal: OpenURI({:?})", uri);
        Ok(())
    }
}
