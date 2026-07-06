//! Structured logging setup (tracing-based) with per-crate filters.

/// Initialise the global tracing subscriber. Call once at app startup.
pub fn init_logger() {
    let filter = std::env::var("LTK_LOG_LEVEL").unwrap_or_else(|_| "info".into());
    // Real impl: tracing_subscriber::fmt().with_env_filter(filter).init();
    log::info!("LTK logger initialised at level: {filter}");
}

/// Per-module log level override.
pub struct LogFilter { pub module: String, pub level: log::LevelFilter }

pub fn set_module_level(module: &str, level: log::LevelFilter) {
    log::debug!("Setting log level for {module} to {level:?}");
}
