//! Capability-based permission system.

use std::collections::{HashMap, HashSet};
use ltk_core::string::SharedString;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Permission {
    ReadFile(String),     // path glob
    WriteFile(String),
    NetworkAccess,
    CameraAccess,
    MicrophoneAccess,
    ClipboardRead,
    ClipboardWrite,
    Notifications,
    LocationAccess,
    Custom(String),
}

#[derive(Debug, Clone, Default)]
pub struct PermissionSet(pub HashSet<Permission>);

impl PermissionSet {
    pub fn grant(&mut self, p: Permission)   { self.0.insert(p); }
    pub fn revoke(&mut self, p: &Permission) { self.0.remove(p); }
    pub fn has(&self, p: &Permission) -> bool { self.0.contains(p) }
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
}

pub struct PermissionManager {
    app_id: SharedString,
    grants: HashMap<SharedString, PermissionSet>,
}

impl PermissionManager {
    pub fn new(app_id: impl Into<String>) -> Self {
        Self { app_id: SharedString::new(app_id), grants: HashMap::new() }
    }

    pub fn grant(&mut self, app: impl Into<String>, perm: Permission) {
        self.grants.entry(SharedString::new(app)).or_default().grant(perm);
    }

    pub fn check(&self, app: &str, perm: &Permission) -> bool {
        self.grants.get(&SharedString::new(app)).map_or(false, |s| s.has(perm))
    }

    pub fn revoke_all(&mut self, app: &str) {
        self.grants.remove(&SharedString::new(app));
    }
}
