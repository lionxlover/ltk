use std::sync::{Arc, RwLock};

/// Theme manager — reads system theme via D-Bus.
///
/// When D-Bus is unavailable (non-Linux), falls back to
/// sensible defaults so the application still works.
pub struct ThemeManager {
    inner: Arc<RwLock<ThemeState>>,
}

#[derive(Clone)]
struct ThemeState {
    dark_mode: bool,
    accent_index: u8,
    font_family: String,
    font_size: f32,
}

impl Default for ThemeState {
    fn default() -> Self {
        Self {
            dark_mode: true,
            accent_index: 0,
            font_family: "Inter".into(),
            font_size: 14.0,
        }
    }
}

const ACCENT_COLORS: &[(u8, u8, u8)] = &[
    (91, 157, 250),  // 0: Blue
    (239, 68, 68),   // 1: Red
    (34, 197, 94),   // 2: Green
    (157, 122, 250), // 3: Purple
    (236, 72, 153),  // 4: Pink
    (249, 115, 22),  // 5: Orange
    (156, 163, 175), // 6: Grey
];

impl ThemeManager {
    pub fn new() -> Self {
        let inner = Arc::new(RwLock::new(ThemeState::default()));

        #[cfg(target_os = "linux")]
        {
            let inner_clone = inner.clone();
            std::thread::spawn(move || {
                Self::dbus_poll_loop(inner_clone);
            });
        }

        Self { inner }
    }

    pub fn dark_mode(&self) -> bool {
        self.inner.read().unwrap().dark_mode
    }

    pub fn set_dark_mode(&self, dark: bool) {
        self.inner.write().unwrap().dark_mode = dark;
    }

    pub fn accent_color(&self) -> (u8, u8, u8) {
        let idx = self.inner.read().unwrap().accent_index as usize;
        ACCENT_COLORS.get(idx).copied().unwrap_or((91, 157, 250))
    }

    pub fn font_family(&self) -> String {
        self.inner.read().unwrap().font_family.clone()
    }

    pub fn font_size(&self) -> f32 {
        self.inner.read().unwrap().font_size
    }

    pub fn downgrade(&self) -> WeakThemeManager {
        WeakThemeManager {
            inner: Arc::downgrade(&self.inner),
        }
    }

    #[cfg(target_os = "linux")]
    fn dbus_poll_loop(inner: Arc<RwLock<ThemeState>>) {
        use std::time::Duration;

        // Try to connect and read initial values
        match Self::dbus_read_initial() {
            Ok((dark, accent, font, size)) => {
                if let Ok(mut state) = inner.write() {
                    state.dark_mode = dark;
                    state.accent_index = accent;
                    state.font_family = font;
                    state.font_size = size;
                }
                log::info!("Connected to system D-Bus theme service");
            }
            Err(e) => {
                log::debug!("D-Bus theme service not available: {}. Using defaults.", e);
                return;
            }
        }

        // Poll for changes
        loop {
            std::thread::sleep(Duration::from_secs(10));
            if let Ok((dark, accent, font, size)) = Self::dbus_read_initial() {
                if let Ok(mut state) = inner.write() {
                    state.dark_mode = dark;
                    state.accent_index = accent;
                    state.font_family = font;
                    state.font_size = size;
                }
            }
        }
    }

    #[cfg(target_os = "linux")]
    fn dbus_read_initial() -> Result<(bool, u8, String, f32), Box<dyn std::error::Error>> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        rt.block_on(Self::dbus_read_async())
    }

    #[cfg(target_os = "linux")]
    async fn dbus_read_async() -> Result<(bool, u8, String, f32), Box<dyn std::error::Error>> {
        use zbus::proxy::Builder as ProxyBuilder;

        let conn = zbus::Connection::session().await?;

        let proxy: zbus::proxy::Proxy<'_> = ProxyBuilder::new(&conn)
            .destination("com.cutefish.Settings")?
            .path("/Theme")?
            .interface("com.cutefish.Theme")?
            .build()
            .await?;

        let dark: bool = proxy.call_method("DarkMode", &()).await
            .map(|r| r.body().deserialize().unwrap_or(true))
            .unwrap_or(true);
        let accent: u8 = proxy.call_method("AccentColor", &()).await
            .map(|r| r.body().deserialize().unwrap_or(0))
            .unwrap_or(0);
        let font: String = proxy.call_method("SystemFont", &()).await
            .map(|r| r.body().deserialize().unwrap_or_else(|_| "Inter".into()))
            .unwrap_or_else(|_| "Inter".into());
        let size: f32 = proxy.call_method("SystemFontPointSize", &()).await
            .map(|r| r.body().deserialize().unwrap_or(14.0))
            .unwrap_or(14.0);

        Ok((dark, accent, font, size))
    }
}

pub struct WeakThemeManager {
    inner: std::sync::Weak<RwLock<ThemeState>>,
}

impl WeakThemeManager {
    pub fn upgrade(&self) -> Option<ThemeManager> {
        self.inner.upgrade().map(|inner| ThemeManager { inner })
    }
}
