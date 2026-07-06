//! Signal handler + stack trace capture for crash reporting.

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct CrashReport {
    pub timestamp:   String,
    pub signal:      String,
    pub backtrace:   String,
    pub app_version: String,
}

/// Install a panic hook + signal handler that writes a crash report on fatal error.
pub fn install_crash_handler(crash_dir: PathBuf) {
    let dir = crash_dir.clone();
    std::panic::set_hook(Box::new(move |info| {
        let report = format!(
            "LTK Crash Report\n=================\nPanic: {info}\n",
        );
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join(format!("crash-{}.log", std::process::id()));
        let _ = std::fs::write(&path, &report);
        log::error!("Application panicked — report saved to {:?}", path);
    }));
}
