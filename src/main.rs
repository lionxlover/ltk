mod thememanager;
mod windowhelper;

use slint::{SharedString, Timer, TimerMode};
use std::time::Duration;

slint::include_modules!();

fn main() {
    env_logger::init();
    log::info!("LTK starting...");

    let app = LtkWindow::new().unwrap();
    let backend = app.global::<Backend>();

    // Set Backend properties
    backend.set_app_name("LTK".into());
    backend.set_app_version("1.0.0".into());
    backend.set_app_description(
        "Lion Toolkit UI component library, Slint + Rust."
            .into(),
    );
    backend.set_app_link("https://github.com/lionxlover/ltk".into());
    backend.set_system_font(SharedString::from("Inter"));
    backend.set_system_font_size(14.0);
    backend.set_device_pixel_ratio(1.0);

    // Theme manager
    let theme_manager = thememanager::ThemeManager::new();

    // Sync initial theme from D-Bus
    let dark = theme_manager.dark_mode();
    app.set_dark_mode(dark);  // two-way binding syncs to Theme.dark-mode
    let accent = theme_manager.accent_color();
    backend.set_system_accent(slint::Color::from_argb_u8(255, accent.0, accent.1, accent.2));
    backend.set_system_font(SharedString::from(&theme_manager.font_family()));
    backend.set_system_font_size(theme_manager.font_size());

    log::info!("Theme: dark={}, accent=rgb({},{},{})", dark, accent.0, accent.1, accent.2);

    // UI callbacks → Rust
    let tm_weak = theme_manager.downgrade();
    let app_weak = app.as_weak();
    app.on_theme_changed(move |dark| {
        log::info!("Theme toggled: dark={}", dark);
        if let Some(tm) = tm_weak.upgrade() {
            tm.set_dark_mode(dark);
        }
    });

    app.on_accent_changed(move |color| {
        if let Some(a) = app_weak.upgrade() {
            a.global::<Backend>().set_system_accent(color);
        }
    });

    let app_weak = app.as_weak();
    app.on_font_changed(move |family, size| {
        if let Some(a) = app_weak.upgrade() {
            let b = a.global::<Backend>();
            b.set_system_font(family);
            b.set_system_font_size(size);
        }
    });

    // X11
    #[cfg(target_os = "linux")]
    {
        windowhelper::setup_window(app.window());
    }

    // D-Bus polling — push theme changes from D-Bus to UI
    let tm_weak = theme_manager.downgrade();
    let app_weak = app.as_weak();
    let poll_timer = Timer::default();
    poll_timer.start(
        TimerMode::Repeated,
        Duration::from_secs(5),
        move || {
            if let (Some(tm), Some(a)) = (tm_weak.upgrade(), app_weak.upgrade()) {
                let dark = tm.dark_mode();
                if a.get_dark_mode() != dark {
                    a.set_dark_mode(dark);  // syncs to Theme.dark-mode
                    log::info!("D-Bus: dark -> {}", dark);
                }

                let accent = tm.accent_color();
                let backend = a.global::<Backend>();
                let cur = backend.get_system_accent();
                let r = (
                    (cur.red() as f32 * 255.0) as u8,
                    (cur.green() as f32 * 255.0) as u8,
                    (cur.blue() as f32 * 255.0) as u8,
                );
                if r != accent {
                    backend.set_system_accent(slint::Color::from_argb_u8(255, accent.0, accent.1, accent.2));
                    log::info!("D-Bus: accent -> rgb({},{},{})", accent.0, accent.1, accent.2);
                }
            }
        },
    );

    log::info!("Running...");
    app.run().unwrap();
}
