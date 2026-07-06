//! Live theme engine: switch, hot-reload, subscriber notification.

use super::Theme;
use ltk_core::{callback::Callback, sync::RwLock};
use std::sync::Arc;

/// Event fired when the active theme changes.
#[derive(Debug, Clone)]
pub struct ThemeChangeEvent {
    pub reason: ThemeChangeReason,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThemeChangeReason {
    ModeSwitch,      // dark ↔ light
    AccentChanged,   // hue changed
    ThemeReplaced,   // whole theme object swapped
    ReduceMotion,    // accessibility preference
    FontSizeChanged, // user preference
}

type ThemeSub = Callback<ThemeChangeEvent>;

/// Central theme engine.  One instance lives for the whole application.
pub struct ThemeEngine {
    theme:      RwLock<Arc<Theme>>,
    subs:       RwLock<Vec<ThemeSub>>,
}

impl ThemeEngine {
    pub fn new(initial: Theme) -> Arc<Self> {
        Arc::new(Self {
            theme: RwLock::new(Arc::new(initial)),
            subs:  RwLock::new(Vec::new()),
        })
    }

    /// Current resolved theme (cheap Arc clone).
    pub fn current(&self) -> Arc<Theme> { self.theme.read().clone() }

    /// Replace the active theme and notify all subscribers.
    pub fn set_theme(&self, theme: Theme, reason: ThemeChangeReason) {
        *self.theme.write() = Arc::new(theme);
        self.notify(ThemeChangeEvent { reason });
    }

    /// Toggle dark/light mode.
    pub fn toggle_mode(&self) {
        let current = self.current();
        let next = match current.mode {
            super::ThemeMode::Dark  => current.source.build_light_theme(&current),
            super::ThemeMode::Light => current.source.build_dark_theme(&current),
            super::ThemeMode::System => return,
        };
        self.set_theme(next, ThemeChangeReason::ModeSwitch);
    }

    /// Change the primary accent hue (0–360°).
    pub fn set_accent_hue(&self, hue: f32) {
        let next = self.current().with_hue(hue);
        self.set_theme(next, ThemeChangeReason::AccentChanged);
    }

    /// Set user font size preference.
    pub fn set_font_scale(&self, scale: f32) {
        let mut t = (*self.current()).clone_metadata();
        t.types = super::TypeScale::new(14.0 * scale);
        self.set_theme(t, ThemeChangeReason::FontSizeChanged);
    }

    /// Subscribe to theme changes. Returns a token; drop it to unsubscribe.
    pub fn subscribe(&self, cb: impl Fn(&ThemeChangeEvent) + Send + Sync + 'static) -> ThemeSub {
        let cb = Callback::new(cb);
        self.subs.write().push(cb.clone());
        cb
    }

    fn notify(&self, event: ThemeChangeEvent) {
        let subs = self.subs.read().clone();
        for sub in &subs { sub.call(&event); }
    }
}

// Helper extension on ThemeColors
trait BuildFromSource {
    fn build_dark_theme(&self, base: &Theme) -> Theme;
    fn build_light_theme(&self, base: &Theme) -> Theme;
}

impl BuildFromSource for crate::color::ThemeColors {
    fn build_dark_theme(&self, base: &Theme) -> Theme {
        Theme {
            name:   base.name.clone(),
            mode:   super::ThemeMode::Dark,
            colors: self.dark.clone(),
            source: self.clone(),
            ..base.clone_metadata()
        }
    }
    fn build_light_theme(&self, base: &Theme) -> Theme {
        Theme {
            name:   base.name.clone(),
            mode:   super::ThemeMode::Light,
            colors: self.light.clone(),
            source: self.clone(),
            ..base.clone_metadata()
        }
    }
}
