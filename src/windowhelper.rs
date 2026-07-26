/// X11 window helper — borderless window management via _NET_WM_MOVERESIZE.
#[cfg(target_os = "linux")]
pub fn setup_window(_window: &slint::Window) {
    log::info!("X11: Slint handles rendering natively via winit/softbuffer");
}
