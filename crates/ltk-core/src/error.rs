//! Unified error type for the entire LTK framework.

use std::fmt;

/// Top-level LTK error.
#[derive(Debug, thiserror::Error)]
pub enum LtkError {
    #[error("Resource not found: {name}")]
    ResourceNotFound { name: String },

    #[error("Widget not found: {id}")]
    WidgetNotFound { id: crate::id::WidgetId },

    #[error("Layout error: {detail}")]
    Layout { detail: String },

    #[error("Render backend error: {detail}")]
    Render { detail: String },

    #[error("Font error: {detail}")]
    Font { detail: String },

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Platform error: {detail}")]
    Platform { detail: String },

    #[error("Permission denied: {action}")]
    PermissionDenied { action: String },

    #[error("Version incompatibility: required {required}, found {found}")]
    VersionMismatch { required: String, found: String },

    #[error("Plugin error [{plugin}]: {detail}")]
    Plugin { plugin: String, detail: String },

    #[error("Accessibility error: {detail}")]
    Accessibility { detail: String },

    #[error("Internal error: {detail}")]
    Internal { detail: String },
}

/// Convenience alias for `Result<T, LtkError>`.
pub type LtkResult<T> = Result<T, LtkError>;

impl LtkError {
    pub fn internal(detail: impl Into<String>) -> Self {
        Self::Internal { detail: detail.into() }
    }
    pub fn platform(detail: impl Into<String>) -> Self {
        Self::Platform { detail: detail.into() }
    }
    pub fn render(detail: impl Into<String>) -> Self {
        Self::Render { detail: detail.into() }
    }
}
